#[derive(Debug, Default, Clone, Copy)]
pub enum DisplayErrorCodes {
    #[default]
    UnableToGetBrightness,
    UnableToSetBrightness,
    Unknown,
}

impl std::fmt::Display for DisplayErrorCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayErrorCodes::UnableToGetBrightness => {
                write!(f, "UnableToGetBrightness")
            }
            DisplayErrorCodes::UnableToSetBrightness => {
                write!(f, "UnableToSetBrightness")
            }
            DisplayErrorCodes::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug)]
pub struct DisplayError {
    pub code: DisplayErrorCodes,
    pub message: String,
}

impl std::fmt::Display for DisplayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.code, self.message)
    }
}

impl DisplayError {
    pub fn new(code: DisplayErrorCodes, message: String) -> Self {
        Self { code, message }
    }
}