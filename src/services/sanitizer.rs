use std::collections::HashMap;

use log::warn;

pub struct Sanitizer {
    bad_chars: HashMap<char, char>,
}

impl Sanitizer {
    fn init() -> HashMap<char, char> {
        HashMap::from([('/', '&')])
    }

    pub fn new() -> Self {
        Self {
            bad_chars: Sanitizer::init(),
        }
    }

    pub fn sanitize_path(&self, path: &str) -> String {
        let mut result = String::from(path);
        let mut did_sanitize = false;
        self.bad_chars.iter().for_each(|(bad_char, replace_with)| {
            result = result.replace(*bad_char, &replace_with.to_string());
            did_sanitize = path.contains(*bad_char);
        });

        if did_sanitize {
            warn!("got path with bad tokens, replaced '{path}' with '{result}'")
        }

        result
    }
}
