#[derive(Debug, Default, Clone, Copy)]
pub enum BluetoothErrorCodes {
    #[default]
    NoBluetoothDeviceFound,
    UnableToTurnOnBluetooth,
    UnableToTurnOffBluetooth,
    UnableToConnectToBluetoothDevice,
    UnableToDisconnectFromBluetoothDevice,
    UnableToGetBluetoothDeviceStatus,
    Unknown,
}

impl std::fmt::Display for BluetoothErrorCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            BluetoothErrorCodes::NoBluetoothDeviceFound => write!(f, "NoBluetoothDeviceFound"),
            BluetoothErrorCodes::UnableToTurnOnBluetooth => write!(f, "UnableToTurnOnBluetooth"),
            BluetoothErrorCodes::UnableToTurnOffBluetooth => write!(f, "UnableToTurnOffBluetooth"),
            BluetoothErrorCodes::UnableToConnectToBluetoothDevice => {
                write!(f, "UnableToConnectToBluetoothDevice")
            }
            BluetoothErrorCodes::UnableToDisconnectFromBluetoothDevice => {
                write!(f, "UnableToDisconnectFromBluetoothDevice")
            }
            BluetoothErrorCodes::UnableToGetBluetoothDeviceStatus => {
                write!(f, "UnableToGetBluetoothDeviceStatus")
            }
            BluetoothErrorCodes::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug)]
pub struct BluetoothError {
    pub code: BluetoothErrorCodes,
    pub message: String,
}

impl std::fmt::Display for BluetoothError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}

impl BluetoothError {
    pub fn new(code: BluetoothErrorCodes, message: String) -> Self {
        BluetoothError { code, message }
    }
}
