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

pub type ProcessResult<T> = Result<T, ProcessError>;

pub fn run_main_process<T, F>(main_process: F) -> ()
where
    F: FnOnce() -> ProcessResult<T> + Sized,
{
    match main_process() {
        Ok(_) => std::process::exit(0),
        Err(process_error) => {
            eprintln!("{}", process_error);
            std::process::exit(process_error.code())
        }
    }
}

#[macro_export]
macro_rules! to_invalid_data {
    ($msg:literal) => {
        |e| ProcessError::InvalidData(format!($msg, e))
    }
}
#[macro_export]
macro_rules! to_unrecoverable {
    ($msg:literal) => {
        |e| ProcessError::Unrecoverable(format!($msg, e))
    }
}
#[macro_export]
macro_rules! to_custom {
    ($msg:literal, $code:expr) => {
        |e| ProcessError::Custom($code, format!($msg, e))
    }
}
