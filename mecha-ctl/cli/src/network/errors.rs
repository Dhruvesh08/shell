#[derive(Debug, Default, Clone, Copy)]
pub enum NetworkErrorCodes {
    #[default]
    UnableToGetNetworkUsage,
    // UnableToGetNetworkInfo,
    UnableToConnectWifi,
    UnableToScanWifi,
    UnableToRemoveWifi,
    Unknown,
}

impl std::fmt::Display for NetworkErrorCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkErrorCodes::UnableToGetNetworkUsage => write!(f, "UnableToGetNetworkUsage"),
            // NetworkErrorCodes::UnableToGetNetworkInfo => write!(f, "UnableToGetNetworkInfo"),
            NetworkErrorCodes::Unknown => write!(f, "Unknown"),
            NetworkErrorCodes::UnableToConnectWifi => write!(f, "UnableToConnectWifi"),
            NetworkErrorCodes::UnableToScanWifi => write!(f, "UnableToScanWifi"),
            NetworkErrorCodes::UnableToRemoveWifi => write!(f, "UnableToRemoveWifi"),
        }
    }
}

#[derive(Debug)]
pub struct NetworkError {
    pub code: NetworkErrorCodes,
    pub message: String,
}

impl std::fmt::Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.code, self.message)
    }
}

impl NetworkError {
    pub fn new(code: NetworkErrorCodes, message: String) -> Self {
        Self { code, message }
    }
}