pub const DIGITS: &[u8; 16] = b"0123456789abcdef";

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Radix {
    Bin = 2,
    Oct = 8,
    Dec = 10,
    Hex = 16,
}

fn estimate_bytes(input_len: usize, radix: Radix) -> usize {
    match radix {
        Radix::Bin => (input_len + 7) / 8,
        Radix::Oct => (input_len * 3 + 7) / 8,
        Radix::Dec => (input_len * 4 + 7) / 8,
        Radix::Hex => (input_len + 1) / 2,
    }
}

fn estimate_digits(bytes_len: usize, radix: Radix) -> usize {
    match radix {
        Radix::Bin => bytes_len * 8,
        Radix::Oct => bytes_len * 3,
        Radix::Dec => bytes_len * 3,
        Radix::Hex => bytes_len * 2,
    }
}

pub fn str_to_bytes(s: &str, radix: Radix) -> Option<Vec<u8>> {
    let mut result = Vec::with_capacity(estimate_bytes(s.len(), radix));
    
    result.push(0u8);

    for ch in s.chars() {
        let mut carry = match ch.to_digit(radix as u32) {
            Some(value) => value as u16,
            None => { return None; }
        };

        for byte in result.iter_mut() {
            let value = (*byte as u16) * radix as u16 + carry;
            *byte = (value % 256) as u8;
            carry = value / 256;
        }

        if carry > 0 {
            result.push(carry as u8);
        }
    }

    Some(result)
}

pub fn bytes_to_str(bytes: &[u8], radix: Radix) -> String {
    let mut digits = Vec::with_capacity(estimate_digits(bytes.len(), radix));

    digits.push(0u8);

    for &byte in bytes.iter().rev() {
        let mut carry = byte as u32;

        for d in digits.iter_mut() {
            let value = (*d as u32) * 256 + carry;
            *d = (value % radix as u32) as u8;
            carry = value / radix as u32;
        }

        while carry > 0 {
            digits.push((carry % radix as u32) as u8);
            carry /= radix as u32;
        }
    }

    digits.iter().rev().map(|&d| DIGITS[d as usize] as char).collect()
}

pub fn convert_all(s: &str, radix: Radix) -> Option<(String, String, String, String)> {
    match str_to_bytes(s, radix) {
        Some(bytes) => {
            Some((
                bytes_to_str(&bytes, Radix::Bin),
                bytes_to_str(&bytes, Radix::Oct),
                bytes_to_str(&bytes, Radix::Dec),
                bytes_to_str(&bytes, Radix::Hex),
            ))
        }
        None => None
    }
}