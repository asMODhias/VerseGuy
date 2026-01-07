use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("External service error: {0}")]
    External(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, AppError>;

impl AppError {
    pub fn context<T>(self, _msg: &str) -> anyhow::Error {
        anyhow::anyhow!("{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_enum_works() {
        let e = AppError::NotFound("user/1".into());
        assert!(format!("{}", e).contains("Not found"));
    }
}
