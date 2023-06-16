use crate::*;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref INCORRECT_PASSWORD: String = response!("403 Forbidden", "{\"error\":\"Forbidden\",\"message\":\"Incorect password\"}", "Content-Type: application/json");
    pub static ref NO_PASSWORD_SPECIFIED: String = response!("400 Bad Request", "{\"error\":\"Bad Request\",\"message\":\"No password specified\"}", "Content-Type: application/json");
    pub static ref NO_USERNAME_SPECIFIED: String = response!("400 Bad Request", "{\"error\":\"Bad Request\",\"message\":\"No username specified\"}", "Content-Type: application/json");
    pub static ref BADLY_FORMATED_HTML: String = response!("400 Bad Request", "{\"error\":\"Bad Request\",\"message\":\"Badly formatted html\"}", "Content-Type: application/json");
    pub static ref BADLY_FORMATED_JSON: String = response!("400 Bad Request", "{\"error\":\"Bad Request\",\"message\":\"Badly formatted json\"}", "Content-Type: application/json");
    pub static ref UNEXPECTED_CONTENT_TYPE: String = response!("400 Bad Request", "{\"error\":\"Bad Request\",\"message\":\"Unexpected content type, please use json\"}", "Content-Type: application/json");
    pub static ref UNKOWN_LENGTH: String = response!("411 Length Required", "{\"error\":\"Length Required\",\"message\":\"Unkown content length\"}", "Content-Type: application/json");
    pub static ref PAYLOAD_TOO_LARGE: String = response!("413 Payload Too Large", format!("{{\"error\":\"Payload Too Large\",\"message\":\"Keep the payload below {PAYLOAD_MAX_LENGTH} bytes\"}}"), "Content-Type: application/json");
    pub static ref HEADERS_TOO_LARGE: String = response!("431 Request Header Fields Too Large", format!("{{\"error\":\"Request Header Fields Too Large\",\"message\":\"Keep the headers below {HEADER_MAX_LENGTH} bytes\"}}"), "Content-Type: application/json");
}

#[macro_export]
macro_rules! response {
    ( $code:expr, $body:expr, $( $header:expr ),* ) => {
        {
            let mut headers = String::new();

            $(
                headers.push_str(&format!("{}\r\n", $header));
            )*

            format!("HTTP/1.1 {}\r\nContent-Length: {}\r\n{}\r\n{}", $code, $body.len(), headers, $body)
        }
    };
}
