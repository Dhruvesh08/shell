use console::Emoji;
use serde::Serialize;

use super::emoji;

pub trait Message {
    fn message(msg: &str);

    fn info(msg: &str, emoji: Option<Emoji>) {
        let msg = match emoji {
            Some(e) => format!("{} {}", e, msg),
            None => format!("{} {}", emoji::INFO, msg),
        };
        Self::message(&msg);
    }

    fn warn(msg: &str) {
        let msg = format!("{} {}", emoji::WARNING, msg);
        Self::message(&msg);
    }

    fn success(msg: &str) {
        let msg = format!("{} {}", emoji::SUCCESS, msg);
        Self::message(&msg);
    }

    #[allow(clippy::wrong_self_convention)]
    fn as_json<T>(value: &T)
    where
        T: ?Sized + Serialize;
}

pub struct StdOut;

impl Message for StdOut {
    fn message(msg: &str) {
        println!("{}", msg);
    }

    fn info(msg: &str, emoji: Option<Emoji>) {
        let msg = match emoji {
            Some(e) => format!("{} {}", e, msg),
            None => format!("{} {}", emoji::INFO, msg),
        };
        Self::message(&msg);
    }

    fn warn(msg: &str) {
        let msg = format!("{} {}", emoji::WARNING, msg);
        Self::message(&msg);
    }

    fn success(msg: &str) {
        let msg = format!("{} {}", emoji::SUCCESS, msg);
        Self::message(&msg);
    }

    fn as_json<T>(value: &T)
    where
        T: ?Sized + Serialize,
    {
        println!("{}", &serde_json::to_string(value).unwrap());
    }
}
