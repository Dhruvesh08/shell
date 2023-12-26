
#[derive(Debug)]
pub enum DeviceInfoCtlErrorCodes {
    UnknownError,
    FailedToGetCpuUsage,
    FailedToGetMemoryUsage,
    FailedToGetDiskUsage,
}

// imple Display for DeviceInfoCtlErrorCodes
impl std::fmt::Display for DeviceInfoCtlErrorCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            DeviceInfoCtlErrorCodes::UnknownError => write!(f, "UnknownError"),
            DeviceInfoCtlErrorCodes::FailedToGetCpuUsage => write!(f, "FailedToGetCpuUsage"),
            DeviceInfoCtlErrorCodes::FailedToGetMemoryUsage => write!(f, "FailedToGetMemoryUsage"),
            DeviceInfoCtlErrorCodes::FailedToGetDiskUsage => write!(f, "FailedToGetDiskUsage"),
        }
    }
}

#[derive(Debug)]
pub struct DeviceInfoCtlError {
    pub code: DeviceInfoCtlErrorCodes,
    pub message: String,
}

// impl Display for DeviceInfoCtlError
impl std::fmt::Display for DeviceInfoCtlError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}

impl DeviceInfoCtlError {
    pub fn new(code: DeviceInfoCtlErrorCodes, message: String) -> Self {
        DeviceInfoCtlError { code, message }
    }
}
