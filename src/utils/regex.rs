use regex::Regex;

const MATCH_U32: &'static str = r"(\d+)";

pub trait StrRegexExtensions {
    fn to_u32(&self) -> Result<u32, ()>;
    fn to_u32_vec(&self) -> Result<Vec<u32>, ()>;
}

impl StrRegexExtensions for &str {
    fn to_u32(&self) -> Result<u32, ()> {
        (Regex::new(MATCH_U32).ok())
            .and_then(|r| r.captures(self))
            .and_then(|c| c.get(1))
            .and_then(|g| g.as_str().parse().ok())
            .map_or(Err(()), |value| Ok(value))
    }

    fn to_u32_vec(&self) -> Result<Vec<u32>, ()> {
        (Regex::new(MATCH_U32).ok())
            .and_then(|regex| {
                Some(
                    regex
                        .captures_iter(self)
                        .into_iter()
                        .filter_map(|captures| captures.get(1))
                        .filter_map(|group| group.as_str().parse::<u32>().ok())
                        .collect::<Vec<u32>>(),
                )
            })
            .map_or(Err(()), |value| Ok(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_to_u32() {
        assert_eq!("".to_u32(), Err(()));
        assert_eq!("1".to_u32(), Ok(1));
        assert_eq!("123".to_u32(), Ok(123));
        assert_eq!("k".to_u32(), Err(()));
        assert_eq!("k123k".to_u32(), Ok(123));
        assert_eq!("123 456".to_u32(), Ok(123));
    }

    #[test]
    fn it_converts_to_u32_vec() {
        assert_eq!("".to_u32_vec(), Ok(vec![]));
        assert_eq!("1".to_u32_vec(), Ok(vec![1]));
        assert_eq!("1 2 3".to_u32_vec(), Ok(vec![1, 2, 3]));
        assert_eq!("1.2.3".to_u32_vec(), Ok(vec![1, 2, 3]));
        assert_eq!("1k2l3".to_u32_vec(), Ok(vec![1, 2, 3]));
    }
}
