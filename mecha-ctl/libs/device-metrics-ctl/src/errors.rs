
#[derive(Debug)]
pub enum DeviceMetricsCtlErrorCodes {
    UnknownError,
    FailedToGetCpuUsage,
    FailedToGetMemoryUsage,
    FailedToGetDiskUsage,
}

// imple Display for DeviceMetricsCtlErrorCodes
impl std::fmt::Display for DeviceMetricsCtlErrorCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            DeviceMetricsCtlErrorCodes::UnknownError => write!(f, "UnknownError"),
            DeviceMetricsCtlErrorCodes::FailedToGetCpuUsage => write!(f, "FailedToGetCpuUsage"),
            DeviceMetricsCtlErrorCodes::FailedToGetMemoryUsage => write!(f, "FailedToGetMemoryUsage"),
            DeviceMetricsCtlErrorCodes::FailedToGetDiskUsage => write!(f, "FailedToGetDiskUsage"),
        }
    }
}

#[derive(Debug)]
pub struct DeviceMetricsCtlError {
    pub code: DeviceMetricsCtlErrorCodes,
    pub message: String,
}

// impl Display for DeviceMetricsCtlError
impl std::fmt::Display for DeviceMetricsCtlError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}

impl DeviceMetricsCtlError {
    pub fn new(code: DeviceMetricsCtlErrorCodes, message: String) -> Self {
        DeviceMetricsCtlError { code, message }
    }
}
