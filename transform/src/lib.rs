pub fn encode(text: &[u8]) -> Vec<u8> {
    text.iter().map(|t| t + 1).collect::<Vec<_>>()
}

pub fn decode(text: &[u8]) -> Vec<u8> {
    text.iter().map(|t| {
        if t > &0 {
            *t - 1
        } else {
            *t
        }
    }).collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_encode() {
        assert_eq!(
            "hello".as_bytes().to_vec(),
            decode(&encode("hello".as_bytes()))
        );
    }
}
