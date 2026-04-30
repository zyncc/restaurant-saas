use axum::{extract::Request, http::header};

pub mod session;

pub fn extract_token(req: &Request) -> Option<String> {
    if let Some(auth) = req.headers().get(header::AUTHORIZATION) {
        if let Ok(val) = auth.to_str() {
            if let Some(token) = val.strip_prefix("Bearer ") {
                return Some(token.to_string());
            }
        }
    }

    if let Some(cookie_header) = req.headers().get(header::COOKIE) {
        if let Ok(cookies) = cookie_header.to_str() {
            for cookie in cookies.split(';') {
                let cookie = cookie.trim();
                if let Some(token) = cookie.strip_prefix("session_token=") {
                    return Some(token.trim().to_string());
                }
            }
        }
    }

    None
}
