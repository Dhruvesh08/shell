#[derive(Debug, Default, Clone, Copy)]

pub enum LedErrorCodes {
    #[default]
    UnableToGetLed,
    UnableToGetLedInfo,
    Unknown,
}

impl std::fmt::Display for LedErrorCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LedErrorCodes::UnableToGetLed => write!(f, "UnableToGetLed"),
            LedErrorCodes::UnableToGetLedInfo => write!(f, "UnableToGetLedInfo"),
            LedErrorCodes::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug)]
pub struct LedError {
    pub code: LedErrorCodes,
    pub message: String,
}

impl std::fmt::Display for LedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.code, self.message)
    }
}

impl LedError {
    pub fn new(code: LedErrorCodes, message: String) -> Self {
        Self { code, message }
    }
}


