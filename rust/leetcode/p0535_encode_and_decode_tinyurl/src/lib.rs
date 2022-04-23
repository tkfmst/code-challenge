/// 535. Encode and Decode TinyURL
/// Medium
///
/// Note: This is a companion problem to the System Design problem: Design TinyURL.
///
/// TinyURL is a URL shortening service where you enter a URL such as https://leetcode.com/problems/design-tinyurl and it returns a short URL such as http://tinyurl.com/4e9iAk. Design a class to encode a URL and decode a tiny URL.
///
/// There is no restriction on how your encode/decode algorithm should work. You just need to ensure that a URL can be encoded to a tiny URL and the tiny URL can be decoded to the original URL.
///
/// Implement the Solution class:
///
/// Solution() Initializes the object of the system.
/// String encode(String longUrl) Returns a tiny URL for the given longUrl.
/// String decode(String shortUrl) Returns the original long URL for the given shortUrl. It is guaranteed that the given shortUrl was encoded by the same object.
///
///
/// Example 1:
///
/// Input: url = "https://leetcode.com/problems/design-tinyurl"
/// Output: "https://leetcode.com/problems/design-tinyurl"
///
/// Explanation:
/// Solution obj = new Solution();
/// string tiny = obj.encode(url); // returns the encoded tiny url.
/// string ans = obj.decode(tiny); // returns the original url after deconding it.
///
///
/// Constraints:
/// * 1 <= url.length <= 10^4
/// * url is guranteed to be a valid URL.
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub struct Codec {
    dict: HashMap<String, String>,
}

impl Codec {
    pub fn new() -> Self {
        Self {
            dict: HashMap::new(),
        }
    }

    // Encodes a URL to a shortened URL.
    pub fn encode(&mut self, long_url: String) -> String {
        let mut hasher = DefaultHasher::new();
        long_url.hash(&mut hasher);
        let s = hasher.finish();
        let hex = format!("{:x}", s);
        self.dict.insert(hex.clone(), long_url);
        format!("http://tinyurl.com/{}", hex)
    }

    // Decodes a shortened URL to its original URL.
    pub fn decode(&mut self, short_url: String) -> String {
        let hex = short_url.replace("http://tinyurl.com/", "");
        match self.dict.get(&hex) {
            Some(url) => url.to_owned(),
            None => panic!("404 Not Found: {}", short_url),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Codec;
    #[test]
    fn test_codec_1() {
        let strs = "https://leetcode.com/problems/design-tinyurl".to_string();
        let mut obj = Codec::new();
        let s: String = obj.encode(strs.clone());
        let ans = obj.decode(s);
        assert_eq!(strs, ans);
    }
}
