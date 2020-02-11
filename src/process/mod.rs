use self::ProcessError::*;

#[derive(Clone, Eq, PartialEq)]
pub enum ProcessError {
    InvalidData(String),
    Unrecoverable(String),
    Custom(i32, String),
}
impl ProcessError {
    fn kind(&self) -> &'static str {
        match self {
            InvalidData(_) => "InvalidData",
            Unrecoverable(_) => "Unrecoverable",
            Custom(_, _) => "Custom",
        }
    }

    fn code(&self) -> i32 {
        match self {
            InvalidData(_) => 1,
            Unrecoverable(_) => 2,
            Custom(code, _) => *code,
        }
    }
}

impl std::fmt::Display for ProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            InvalidData(msg) => write!(f, "{}", msg),
            Unrecoverable(msg) => write!(f, "{}", msg),
            Custom(code, msg) => write!(f, "Error code {}: {}", code, msg),
        }
    }
}

impl std::fmt::Debug for ProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}({})", self.kind(), self)
    }
}

impl std::error::Error for ProcessError {}

pub type ProcessResult = Result<(), ProcessError>;

pub fn run_main_process<F>(main_process: F) -> ()
where
    F: FnOnce() -> ProcessResult + Sized,
{
    match main_process() {
        Ok(_) => std::process::exit(0),
        Err(process_error) => {
            eprintln!("{}", process_error);
            std::process::exit(process_error.code())
        }
    }
}
