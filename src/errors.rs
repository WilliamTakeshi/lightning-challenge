use reqwest::StatusCode;

pub fn internal_error() -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Internal Server Error".to_string(),
    )
}

pub fn unprocessable_entity<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::UNPROCESSABLE_ENTITY, err.to_string())
}

pub fn not_found<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::NOT_FOUND, err.to_string())
}
