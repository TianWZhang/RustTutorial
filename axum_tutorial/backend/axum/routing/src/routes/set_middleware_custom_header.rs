use axum::{extract::Request, middleware::Next, response::Response};

use super::read_middleware_custom_header::HeaderMessage;

pub async fn set_middleware_custom_header(
    mut request: Request,
    next: Next,
) -> Response {
    let headers = request.headers();
    let message = headers.get("message").unwrap();
    let message = message.to_str().unwrap().to_owned();

    let extensions = request.extensions_mut();
    extensions.insert(HeaderMessage(message));
    
    next.run(request).await
}