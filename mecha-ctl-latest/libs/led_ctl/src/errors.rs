#[derive(Debug)]
pub enum LedctlErrorCodes {
    InvalidLedColorError,
    InvalidLedPathValueError,
}

#[derive(Debug)]
pub struct LedctlError {
    pub code: LedctlErrorCodes,
    pub message: String,
}

impl std::fmt::Display for LedctlErrorCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            LedctlErrorCodes::InvalidLedPathValueError => {
                write!(f, "InvalidBrightnessValueError")
            }
            LedctlErrorCodes::InvalidLedColorError => {
                write!(f, "InvalidBrightnessPathError")
            }
        }
    }
}

impl std::fmt::Display for LedctlError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}

impl LedctlError {
    pub fn new(code: LedctlErrorCodes, message: String) -> Self {
        LedctlError { code, message }
    }
}
