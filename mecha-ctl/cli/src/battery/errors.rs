#[derive(Debug, Default, Clone, Copy)]
pub enum BatteryErrorCodes {
    #[default]
    Unknown,
    UnableToDetectBattery,
    UnableToGetBatteryInfo,
}

impl std::fmt::Display for BatteryErrorCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BatteryErrorCodes::Unknown => write!(f, "Unknown"),
            BatteryErrorCodes::UnableToDetectBattery => write!(f, "UnableToDetectBattery"),
            BatteryErrorCodes::UnableToGetBatteryInfo => write!(f, "UnableToGetBatteryInfo"),
        }
    }
}

#[derive(Debug)]
pub struct BatteryError {
    pub code: BatteryErrorCodes,
    pub message: String,
}

impl std::fmt::Display for BatteryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.code, self.message)
    }
}

impl BatteryError {
    pub fn new(code: BatteryErrorCodes, message: String) -> Self {
        Self { code, message }
    }
}
