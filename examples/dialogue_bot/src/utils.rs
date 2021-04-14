use lazy_static::lazy_static;
use regex::Regex;

pub fn extract_bot_command(text: &str) -> Option<String> {
    lazy_static! {
        static ref HASHTAG_REGEX : Regex = Regex::new(
                r"/\S*"
            ).unwrap();
    }
    let h: Vec<String> = HASHTAG_REGEX.find_iter(text).map(|mat| mat.as_str().replace("/", "")).collect();
    if h.len() > 0 {
        Some(h[0].clone())
    } else {
        None
    }
}
