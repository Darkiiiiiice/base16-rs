
const BASE16_UPPER_TABLE: [u8; 16] = *b"0123456789ABCDEF";
const BASE16_LOWER_TABLE: [u8; 16] = *b"0123456789abcdef";


pub struct Encoder {
    typ: Type,
}

impl Encoder {
    pub fn new(typ: Type) -> Self {
        Encoder { typ }
    }
}

impl Encoder {
    pub fn encode_upper_to_string(&self, src: &[u8]) -> String {
        match self.typ {
            Type::StdEncoding => self.encode_upper_std(src).into_iter().map(|b| b as char).collect(),
        }
    }

    pub fn encode_lower_to_string(&self, src: &[u8]) -> String {
        match self.typ {
            Type::StdEncoding => self.encode_lower_std(src).into_iter().map(|b| b as char).collect(),
        }
    }

    #[inline]
    fn encode_upper_std(&self, src: &[u8]) -> Vec<u8> {
        self.encode_std(src, EncodeConfig::UPPER)
    }

    #[inline]
    fn encode_lower_std(&self, src: &[u8]) -> Vec<u8> {
        self.encode_std(src, EncodeConfig::LOWER)
    }

    #[inline]
    fn encode_std(&self, src: &[u8], cfg: EncodeConfig) -> Vec<u8> {
        let table = if cfg == EncodeConfig::LOWER { BASE16_LOWER_TABLE } else { BASE16_UPPER_TABLE };

        let mut dst = Vec::with_capacity(src.len() * 2);
        for b in src.iter() {
            let byte = *b;
            dst.push(table[(byte >> 4) as usize]);
            dst.push(table[(byte & 0x0F) as usize]);
        }
        dst
    }
}

pub enum Type {
    StdEncoding,
}

#[derive(PartialEq)]
enum EncodeConfig {
    UPPER,
    LOWER,
}


#[cfg(test)]
mod encode_tests {
    use crate::base16::Type::StdEncoding;
    use super::*;

    #[test]
    fn empty() {
        let buf: Vec<u8> = Vec::from("");
        let encoder = Encoder::new(StdEncoding);
        let result  = encoder.encode_upper_to_string(&buf);
        assert_eq!(result, "");
    }

    #[test]
    fn number() {
        let buf: Vec<u8> = Vec::from("0123456789");
        let encoder = Encoder::new(StdEncoding);
        let result  = encoder.encode_upper_to_string(&buf);
        assert_eq!(result, "30313233343536373839");
    }

    #[test]
    fn alpha() {
        let buf: Vec<u8> = Vec::from("abcdefghijklnmopqrstuvwxyz");
        let encoder = Encoder::new(StdEncoding);
        let result  = encoder.encode_upper_to_string(&buf);
        assert_eq!(result, "6162636465666768696A6B6C6E6D6F707172737475767778797A");
    }

    #[test]
    fn enter() {
        let buf: Vec<u8> = Vec::from(r#"abcdefghijklnmo
pqrstuvwxyz"#);
        let encoder = Encoder::new(StdEncoding);
        let result  = encoder.encode_upper_to_string(&buf);
        assert_eq!(result, "6162636465666768696A6B6C6E6D6F0A707172737475767778797A");
    }
}
