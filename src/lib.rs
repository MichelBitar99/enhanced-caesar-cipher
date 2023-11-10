use std::collections::HashMap;
use std::collections::VecDeque;

pub fn decrypt<'a>(key: &'a str, cipher: &'a str) -> String {
    // build mapping table
    let mut map = HashMap::new();
    let mut letters: VecDeque<char> = ('a'..='z').collect();
    for k in key.chars() {
        if map.get(&k).is_some() || k == ' ' {
            continue;
        } else {
            map.insert(k, letters.pop_front().unwrap());
        }
    }
    dbg!(&map);

    // decode
    cipher
        .chars()
        .map(|x| *map.get(&x).unwrap_or(&x))
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let key = "eljuxhpwnyrdgtqkviszcfmabo".into();
        let cipher = "zwx hnfx lqantp mnoeius ycgk vcnjrdb".into();
        let clear = "the five boxing wizards jump quickly";
        assert_eq!(decrypt(key, cipher), clear)
    }

    #[test]
    fn complex() {
        let key = "the quick brown fox jumps over the lazy dog".into();
        let cipher = "vkbs bs t suepuv".into();
        let clear = "this is a secret";
        assert_eq!(decrypt(key, cipher), clear)
    }

    #[test]
    fn edge() {
        let key = "the quick brown fox jumps over the lazy dog".into();
        let cipher = "vkbs-bs-t-suepuv".into();
        let clear = "this-is-a-secret";
        assert_eq!(decrypt(key, cipher), clear)
    }

    #[test]
    fn empty_message() {
        let key = "the quick brown fox jumps over the lazy dog".into();
        let cipher = " ".into();
        let clear = " ";
        assert_eq!(decrypt(key, cipher), clear)
    }
}
