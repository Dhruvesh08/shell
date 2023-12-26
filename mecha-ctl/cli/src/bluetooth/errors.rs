#[derive(Debug, Default, Clone, Copy)]
pub enum BluetoothErrorCodes {
    #[default]
    UnableToDetectBluetooth,
    UnableToConnectBluetooth,
    UnableToScanBluetooth,
    UnableToRemoveBluetooth,
    Unknown,
}

impl std::fmt::Display for BluetoothErrorCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BluetoothErrorCodes::UnableToDetectBluetooth => write!(f, "UnableToDetectBluetooth"),
            BluetoothErrorCodes::Unknown => write!(f, "Unknown"),
            BluetoothErrorCodes::UnableToConnectBluetooth => write!(f, "UnableToConnectBluetooth"),
            BluetoothErrorCodes::UnableToScanBluetooth => write!(f, "UnableToScanBluetooth"),
            BluetoothErrorCodes::UnableToRemoveBluetooth => write!(f, "UnableToRemoveBluetooth"),
        }
    }
}

#[derive(Debug)]
pub struct BluetoothError {
    pub code: BluetoothErrorCodes,
    pub message: String,
}

impl std::fmt::Display for BluetoothError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.code, self.message)
    }
}

impl BluetoothError {
    pub fn new(code: BluetoothErrorCodes, message: String) -> Self {
        Self { code, message }
    }
}
