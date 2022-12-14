use mobc_redis::mobc;

#[derive(Debug, thiserror::Error)]
pub enum ApartmentError {
    #[error("Redis error")]
    RedisError(#[from] mobc_redis::redis::RedisError),
    #[error("Mobc redis pool error")]
    MobcRedisError(#[from] mobc::Error<mobc_redis::redis::RedisError>),
    #[error("Serde json error")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("Reqwest error")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Regex error")]
    RegexError(#[from] regex::Error),
}

pub type ApartmentResult<T> = Result<T, ApartmentError>;
