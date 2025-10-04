use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct AuthError {
    error: String,
}

/// Extract Bearer token from Authorization header
fn extract_bearer_token(headers: &HeaderMap) -> Result<String, Response> {
    let auth_header = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                axum::Json(AuthError {
                    error: "Missing Authorization header".to_string(),
                }),
            )
                .into_response()
        })?;

    if !auth_header.starts_with("Bearer ") {
        return Err((
            StatusCode::UNAUTHORIZED,
            axum::Json(AuthError {
                error: "Invalid Authorization header format. Expected 'Bearer <token>'".to_string(),
            }),
        )
            .into_response());
    }

    Ok(auth_header[7..].to_string())
}

/// Middleware to require valid admin API key
pub async fn require_admin_api_key(
    State(admin_api_key_hash): State<String>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Response {
    let api_key = match extract_bearer_token(&headers) {
        Ok(key) => key,
        Err(response) => return response,
    };

    tracing::debug!("Extracted API key: {}", api_key);
    tracing::debug!("Hash from config: {}", admin_api_key_hash);

    // Verify the API key against the hash
    match bcrypt::verify(&api_key, &admin_api_key_hash) {
        Ok(true) => {
            tracing::info!("API key verification successful");
            // API key is valid, proceed with the request
            next.run(request).await
        }
        Ok(false) => {
            tracing::warn!("API key verification failed - key does not match hash");
            (
                StatusCode::UNAUTHORIZED,
                axum::Json(AuthError {
                    error: "Invalid API key".to_string(),
                }),
            )
                .into_response()
        }
        Err(e) => {
            tracing::error!("Error verifying API key: {}", e);
            tracing::error!("API key length: {}, Hash length: {}", api_key.len(), admin_api_key_hash.len());
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(AuthError {
                    error: "Authentication error".to_string(),
                }),
            )
                .into_response()
        }
    }
}
