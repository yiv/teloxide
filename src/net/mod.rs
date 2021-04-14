#[cfg(feature = "unstable-stream")]
pub use download::download_file_stream;

pub use self::{
    download::download_file,
    request::{request_json, request_multipart},
    telegram_response::TelegramResponse,
};

mod download;
mod request;
mod telegram_response;

use std::env;

const TELEGRAM_API_URL: &str = "https://api.telegram.org";

/// Creates URL for making HTTPS requests. See the [Telegram documentation].
///
/// [Telegram documentation]: https://core.telegram.org/bots/api#making-requests
fn method_url(token: &str, method_name: &str) -> String {
    let base = env::var("TELEGRAM_API_URL").ok().unwrap_or(TELEGRAM_API_URL.to_string());
    format!(
        "{url}/bot{token}/{method}",
        url = base,
        token = token,
        method = method_name,
    )
}

/// Creates URL for downloading a file. See the [Telegram documentation].
///
/// [Telegram documentation]: https://core.telegram.org/bots/api#file
fn file_url(token: &str, file_path: &str) -> String {
    let base = env::var("TELEGRAM_API_URL").ok().unwrap_or(TELEGRAM_API_URL.to_string());
    format!(
        "{url}/file/bot{token}/{file}",
        url = base,
        token = token,
        file = file_path,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn method_url_test() {
        let url = method_url(
            "535362388:AAF7-g0gYncWnm5IyfZlpPRqRRv6kNAGlao",
            "methodName",
        );

        assert_eq!(
            url,
            "https://api.telegram.org/bot535362388:AAF7-g0gYncWnm5IyfZlpPRqRRv6kNAGlao/methodName"
        );
    }

    #[test]
    fn file_url_test() {
        let url = file_url(
            "535362388:AAF7-g0gYncWnm5IyfZlpPRqRRv6kNAGlao",
            "AgADAgADyqoxG2g8aEsu_KjjVsGF4-zetw8ABAEAAwIAA20AA_8QAwABFgQ",
        );

        assert_eq!(
            url,
            "https://api.telegram.org/file/bot535362388:AAF7-g0gYncWnm5IyfZlpPRqRRv6kNAGlao/AgADAgADyqoxG2g8aEsu_KjjVsGF4-zetw8ABAEAAwIAA20AA_8QAwABFgQ"
        );
    }
}
