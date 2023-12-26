#[derive(Debug)]
pub enum MotionSensorControlErrorCodes {
    NoMotionDetected,
    UnableToReadMotionSensorControl,
    Unknown,
    UnableToOpenFile,
    UnableToParseValue,
}

impl std::fmt::Display for MotionSensorControlErrorCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            MotionSensorControlErrorCodes::NoMotionDetected => write!(f, "NoMotionDetected"),
            MotionSensorControlErrorCodes::UnableToReadMotionSensorControl => write!(f, "UnableToReadMotionSensorControl"),
            MotionSensorControlErrorCodes::Unknown => write!(f, "Unknown"),
            MotionSensorControlErrorCodes::UnableToOpenFile => write!(f, "UnableToOpenFile"),
            MotionSensorControlErrorCodes::UnableToParseValue => write!(f, "UnableToParseValue"),
        }
    }
}

#[derive(Debug)]
pub struct MotionSensorControlError {
    pub code: MotionSensorControlErrorCodes,
    pub message: String,
}

impl std::fmt::Display for MotionSensorControlError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}

impl MotionSensorControlError {
    pub fn new(code: MotionSensorControlErrorCodes, message: String) -> Self {
        MotionSensorControlError { code, message }
    }
}
