#[derive(Debug)]
pub enum CpuGovernanceCtlErrorCodes {
    FailedToSetCpuGovernor,
    FailedToSetCpuGovernorPath,
    FailedToGetCpuGovernor,
    FailedToGetCpuFrequency,
    FailedToSetCpuFrequency,
    FailedToSetCpuFrequencyPath,
    FailedToOpenFile,
    FailedToWriteToFile,
    FailedToReadFile,
    UnknownError,
}

//impl std::fmt::Display  for CpuGovernanceCtlErrorCodes
impl std::fmt::Display for CpuGovernanceCtlErrorCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            CpuGovernanceCtlErrorCodes::FailedToSetCpuGovernor => write!(f, "FailedToSetCpuGovernor"),
            CpuGovernanceCtlErrorCodes::FailedToSetCpuGovernorPath => {
                write!(f, "FailedToSetCpuGovernorPath")
            }
            CpuGovernanceCtlErrorCodes::FailedToGetCpuGovernor => write!(f, "FailedToGetCpuGovernor"),
            CpuGovernanceCtlErrorCodes::FailedToGetCpuFrequency => write!(f, "FailedToGetCpuFrequency"),
            CpuGovernanceCtlErrorCodes::FailedToSetCpuFrequency => write!(f, "FailedToSetCpuFrequency"),
            CpuGovernanceCtlErrorCodes::FailedToSetCpuFrequencyPath => {
                write!(f, "FailedToSetCpuFrequencyPath")
            }
            CpuGovernanceCtlErrorCodes::FailedToOpenFile => write!(f, "FailedToOpenFile"),
            CpuGovernanceCtlErrorCodes::FailedToWriteToFile => write!(f, "FailedToWriteToFile"),
            CpuGovernanceCtlErrorCodes::FailedToReadFile => write!(f, "FailedToReadFile"),
            CpuGovernanceCtlErrorCodes::UnknownError => write!(f, "UnknownError"),
        }
    }
}

#[derive(Debug)]
pub struct CpuGovernanceCtlError {
    pub code: CpuGovernanceCtlErrorCodes,
    pub message: String,
}

//impl std::fmt::Display for CpuGovernanceCtlError
impl std::fmt::Display for CpuGovernanceCtlError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}

impl CpuGovernanceCtlError {
    pub fn new(code: CpuGovernanceCtlErrorCodes, message: String) -> Self {
        CpuGovernanceCtlError { code, message }
    }
}
