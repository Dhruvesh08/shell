use std::fmt;

use gtk::{glib::clone, prelude::GtkWindowExt};
use relm4::gtk::LayoutManager;
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, SimpleComponent};
use relm4::{Component, ComponentController, Controller};

mod pages;
mod settings;
mod theme;
mod widgets;
use pages::{
    home::{HomePage, Message as HomePageMessage, Settings as HomePageSettings},
    password_authentication::{
        Message as PasswordAuthenticationMessage, PasswordAuthentication,
        Settings as PasswordAuthenticationSettings,
    },
    pin_authentication::{
        Message as PinAuthenticationMessage, PinAuthentication,
        Settings as PinAuthenticationSettings,
    },
};
use tracing::{error, info};
pub mod errors;

use crate::settings::LockScreenSettings;
use crate::theme::LockScreenTheme;

/// # LockScreen State
///
/// This struct is the state definition of the entire application
struct LockScreen {
    settings: LockScreenSettings,
    custom_theme: LockScreenTheme,
    current_screen: Screens,
    home_page: Controller<HomePage>,
    pin_authentication_page: Controller<PinAuthentication>,
    password_authentication_page: Controller<PasswordAuthentication>,
}

#[derive(Debug, Clone)]
pub enum Screens {
    LockScreen,
    PasswordScreen,
    PinScreen,
}

impl fmt::Display for Screens {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Screens::LockScreen => write!(f, "lock_screen"),
            Screens::PasswordScreen => write!(f, "password_screen"),
            Screens::PinScreen => write!(f, "pin_screen"),
        }
    }
}

/// ## Message
///
/// These are the events (or messages) that update state.
/// Each of them are handled in the ``impl Application()::update()``
#[derive(Debug, Clone)]
pub enum Message {
    ChangeScreen(Screens),
    Dummy,
}

struct AppWidgets {
    screens_stack: gtk::Stack,
}

#[cfg(not(feature = "layer-shell"))]
fn init_window(settings: LockScreenSettings) -> gtk::Window {
    let window_settings = settings.window;
    let window = gtk::Window::builder()
        .title(settings.title)
        .default_width(window_settings.size.0)
        .default_height(window_settings.size.1)
        .css_classes(["window"])
        .build();
    window
}

#[cfg(feature = "layer-shell")]
fn init_window(settings: LockScreenSettings) -> gtk::Window {
    let window_settings = settings.window;
    let window = gtk::Window::builder()
        .title(settings.title)
        .default_width(window_settings.size.0)
        .default_height(window_settings.size.1)
        .css_classes(["window"])
        .build();

    gtk4_layer_shell::init_for_window(&window);

    // Display above normal windows
    gtk4_layer_shell::set_layer(&window, gtk4_layer_shell::Layer::Top);

    // The margins are the gaps around the window's edges
    // Margins and anchors can be set like this...
    gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Left, 0);
    gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Right, 0);
    gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Top, 0);
    gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Bottom, 0);

    gtk4_layer_shell::set_keyboard_mode(&window, gtk4_layer_shell::KeyboardMode::OnDemand);

    // ... or like this
    // Anchors are if the window is pinned to each edge of the output
    let anchors = [
        (gtk4_layer_shell::Edge::Left, true),
        (gtk4_layer_shell::Edge::Right, true),
        (gtk4_layer_shell::Edge::Top, true),
        (gtk4_layer_shell::Edge::Bottom, true),
    ];

    for (anchor, state) in anchors {
        gtk4_layer_shell::set_anchor(&window, anchor, state);
    }

    window
}

impl SimpleComponent for LockScreen {
    /// The type of the messages that this component can receive.
    type Input = Message;
    /// The type of the messages that this component can send.
    type Output = ();
    /// The type of data with which this component will be initialized.
    type Init = ();
    /// The root GTK widget that this component will create.
    type Root = gtk::Window;
    /// A data structure that contains the widgets that you will need to update.
    type Widgets = AppWidgets;

    fn init_root() -> Self::Root {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => LockScreenSettings::default(),
        };

        info!(
            task = "initalize_settings",
            "settings initialized for Lock Screen: {:?}", settings
        );

        let custom_theme = match theme::read_theme_yml() {
            Ok(theme) => theme,
            Err(_) => LockScreenTheme::default(),
        };

        info!(
            task = "initalize_theme",
            "theme initialized for Lock Screen: {:?}", custom_theme
        );

        let window = init_window(settings);
        window
    }

    /// Initialize the UI and model.
    fn init(
        _: Self::Init,
        window: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let icon_theme = gtk::IconTheme::builder().build();
        info!("icon paths are {:?}", icon_theme.resource_path());
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => LockScreenSettings::default(),
        };

        let css = settings.css.clone();
        relm4::set_global_css_from_file(css.default);

        let custom_theme = match theme::read_theme_yml() {
            Ok(theme) => theme,
            Err(_) => LockScreenTheme::default(),
        };

        let modules = settings.modules.clone();
        let layout = settings.layout.clone();

        //Stack used to render different screens
        //At a time one screen will be rendered
        let screens_stack = gtk::Stack::builder().build();

        let home_page = HomePage::builder()
            .launch(HomePageSettings {
                lock_icon: modules.lock.icon.default.to_owned(),
                unlock_icon: modules.unlock.icon.default.to_owned(),
                password_icon: modules.home_password.icon.default.to_owned(),
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| match msg {
                    HomePageMessage::ChangeScreen(screen) => Message::ChangeScreen(screen)
                }),
            );

        let pin_authentication_page = PinAuthentication::builder()
            .launch(PinAuthenticationSettings {
                modules: modules.clone(),
                layout: layout.clone(),
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| {
                    info!("auth page message to parent {:?}", msg);
                    match msg {
                       PinAuthenticationMessage::HomeIconPressed => Message::ChangeScreen(Screens::LockScreen),
                        _ => Message::Dummy
                    }
                }),
            );

        let password_authentication_page = PasswordAuthentication::builder()
            .launch(PasswordAuthenticationSettings {
                modules: modules.clone(),
                layout: layout.clone(),
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| {
                    info!("auth page message to parent {:?}", msg);
                    match msg {
                        PasswordAuthenticationMessage::BackPressed => Message::ChangeScreen(Screens::LockScreen),
                        _ => Message::Dummy
                    }
                }),
            );

        //Adding home page screeen in stack
        screens_stack.add_named(
            home_page.widget(),
            Option::from(Screens::LockScreen.to_string().as_str()),
        );

        //Adding auth screeen in stack
        screens_stack.add_named(
            pin_authentication_page.widget(),
            Option::from(Screens::PinScreen.to_string().as_str()),
        );

        //Adding password screeen in stack
        screens_stack.add_named(
            password_authentication_page.widget(),
            Option::from(Screens::PasswordScreen.to_string().as_str()),
        );

        let current_screen = Screens::LockScreen;

        //Setting current active screen in stack
        screens_stack.set_visible_child_name(&current_screen.to_string());

        //Adding stack to window
        window.set_child(Some(&screens_stack));

        let model = LockScreen {
            settings,
            custom_theme,
            current_screen,
            home_page,
            pin_authentication_page,
            password_authentication_page,
        };

        let widgets = AppWidgets { screens_stack };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        info!("Update message is {:?}", message);
        match message {
            Message::ChangeScreen(screen) => {
                self.current_screen = screen;
            }
            _ => (),
        }
    }

    /// Update the view to represent the updated model.
    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        //updating stack screen when current screen changes
        widgets
            .screens_stack
            .set_visible_child_name(self.current_screen.to_string().as_str());
    }
}

fn main() {
    // Enables logger
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter("mecha_lock_screen=trace")
        .with_thread_names(true)
        .init();
    let app = RelmApp::new("lock.screen").with_args(vec![]);
    app.run::<LockScreen>(());
}
