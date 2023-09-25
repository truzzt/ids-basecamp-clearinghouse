type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    Generic(anyhow::Error),
}

impl std::error::Error for AppError {}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::Generic(e) => write!(f, "{}", e),
        }
    }
}

impl From<anyhow::Error> for AppError{
    fn from(err: anyhow::Error) -> Self {
        Self::Generic(err.into())
    }
}