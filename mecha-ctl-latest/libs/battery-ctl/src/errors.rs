#[derive(Debug)]
pub enum PowerSupplyErrorCodes {
    FailedToOpenFile,
    FailedToReadFile,
    InvalidDataFormat,
    UnknownError,
}

impl std::fmt::Display for PowerSupplyErrorCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            PowerSupplyErrorCodes::FailedToOpenFile => write!(f, "FailedToOpenFile"),
            PowerSupplyErrorCodes::FailedToReadFile => write!(f, "FailedToReadFile"),
            PowerSupplyErrorCodes::InvalidDataFormat => write!(f, "InvalidDataFormat"),
            PowerSupplyErrorCodes::UnknownError => write!(f, "UnknownError"),
        }
    }
}

#[derive(Debug)]
pub struct PowerSupplyError {
    pub code: PowerSupplyErrorCodes,
    pub message: String,
}

impl std::fmt::Display for PowerSupplyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}

impl PowerSupplyError {
    pub fn new(code: PowerSupplyErrorCodes, message: String) -> Self {
        PowerSupplyError { code, message }
    }
}
