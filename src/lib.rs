#![cfg_attr(windows, feature(abi_vectorcall))]
use ext_php_rs::{prelude::*, binary::Binary, zend::ce};
use faster_hex::{hex_string, hex_decode};

#[php_class(name = "Muvon\\Ext\\VarIntError")]
#[extends(ce::exception())]
pub struct VarIntError;

#[php_class(name = "Muvon\\Ext\\VarInt")]
pub struct VarInt;

#[php_impl(rename_methods = "camelCase")]
impl VarInt {
    // We use String here because PHP_INT_MAX is signed integer
    // But we are working with unsigned here
    #[public]
    pub fn pack_uint(value: &str) -> Binary<u8> {
        let value = value.parse::<u64>().expect("Failed to parse input into uint64");
        Self::real_pack_uint(value)
    }

    // Helper function to relfect u64 instead of &str for php
    fn real_pack_uint(value: u64) -> Binary<u8> {
        let mut h: Vec<u8> = vec![];
        let mut value = value;
        while value >= 0x80 {
            let u = (value & 0xff | 0x80) as u8;
            h.push(u);
            value = value >> 7;
        }

        h.push((value as u8) & 0xff);
        Binary::new(h)
    }

    #[public]
    pub fn pack_uint_hex(value: &str) -> String {
        hex_string(Self::pack_uint(value).as_slice())
    }

    #[public]
    pub fn pack_int(value: i64) -> Binary<u8> {
        let mut ux: u64 = (value as u64) << 1u64;
        if value < 0 {
            ux = !ux;
        }
        Self::real_pack_uint(ux)
    }

    #[public]
    pub fn pack_int_hex(value: i64) -> String {
        hex_string(Self::pack_int(value).as_slice())
    }

    fn real_read_uint(bin: Binary<u8>, offset: Option<u64>, max_len: Option<u64>) -> Result<Vec<u64>, PhpException> {
        let offset = offset.unwrap_or(0);
        let max_len = max_len.unwrap_or(0);

        let mut x: u64 = 0;
        let mut s: u64 = 0;
        let mut i: u64 = offset;
        let max_i = max_len + offset;

        for u in bin.iter() {
            let u = u.to_owned();
            if u < 0x80 {
                if max_len > 0 && (i > max_i || i == max_i && u > 1) {
                    return Err(PhpException::from_class::<VarIntError>("Value overwlow".into()));
                }
                return Ok(vec![(x | (u as u64) << s) as u64, i + 1]);
            }
            x = x | ((u & 0x7f) as u64) << s;
            s += 7;
            i += 1;
        }

        Ok(vec![0, 0])
    }

    #[public]
    pub fn read_uint(bin: Binary<u8>, offset: Option<u64>, max_len: Option<u64>) -> Result<Vec<String>, PhpException> {
        match Self::real_read_uint(bin, offset, max_len) {
            Ok(res) => Ok(res.iter().map(|x| x.to_string()).collect()),
            Err(err) => Err(err),
        }
    }

    #[public]
    pub fn read_uint_hex(hex: String, offset: Option<u64>, max_len: Option<u64>) -> Result<Vec<String>, PhpException> {
        let bin = Self::try_hex_to_binary(hex)?;
        Self::read_uint(bin, offset, max_len)
    }

    #[public]
    pub fn read_int(bin: Binary<u8>, offset: Option<u64>, max_len: Option<u64>) -> Result<Vec<i64>, PhpException> {
        let input = Self::real_read_uint(bin, offset, max_len)?;
        let (ux, offset) = match input[..] {
            [a, b] => (a, b),
            _ => unreachable!(),
        };

        let mut x = (ux >> 1) as i64;
        let b = ux & 1;
        if b != 0 {
            x = !x;
        }

        Ok(vec![x, offset as i64])
    }

    #[public]
    pub fn read_int_hex(hex: String, offset: Option<u64>, max_len: Option<u64>) -> Result<Vec<i64>, PhpException> {
        let bin = Self::try_hex_to_binary(hex)?;
        Self::read_int(bin, offset, max_len)
    }

    #[public]
    pub fn read_bool(bin: Binary<u8>, offset: Option<u64>) -> Result<Vec<u64>, PhpException> {
        let result = Self::read_uint(bin, offset, Some(0u64))?;
        Ok(result.iter().map(|x| x.parse::<u64>().unwrap_or(0)).collect())
    }

    #[public]
    pub fn read_bool_hex(hex: String, offset: Option<u64>) -> Result<Vec<u64>, PhpException> {
        let bin = Self::try_hex_to_binary(hex)?;
        Self::read_bool(bin, offset)
    }

    fn try_hex_to_binary(hex: String) -> Result<Binary<u8>, PhpException> {
        let len = hex.len() / 2;
        let mut dst = Vec::with_capacity(len);
        dst.resize(len, 0);
        match hex_decode(hex.as_bytes(), &mut dst) {
            Ok(_) => (),
            Err(_) => return Err(PhpException::from_class::<VarIntError>("Failed to decode hex input".into())),
        };
        Ok(Binary::new(dst))
    }
}

#[php_function]
pub fn varint_pack_uint(value: &str) -> Binary<u8> {
    VarInt::pack_uint(value)
}

#[php_function]
pub fn varint_pack_int(value: i64) -> Binary<u8> {
    VarInt::pack_int(value)
}

#[php_function]
pub fn varint_read_uint(bin: Binary<u8>, offset: Option<u64>, max_len: Option<u64>) -> Result<Vec<String>, PhpException> {
    VarInt::read_uint(bin, offset, max_len)
}

#[php_function]
pub fn varint_read_int(bin: Binary<u8>, offset: Option<u64>, max_len: Option<u64>) -> Result<Vec<i64>, PhpException> {
    VarInt::read_int(bin, offset, max_len)
}

#[php_function]
pub fn varint_read_int_hex(hex: String, offset: Option<u64>, max_len: Option<u64>) -> Result<Vec<i64>, PhpException> {
    VarInt::read_int_hex(hex, offset, max_len)
}

#[php_function]
pub fn varint_read_bool(bin: Binary<u8>, offset: Option<u64>) -> Result<Vec<u64>, PhpException> {
    VarInt::read_bool(bin, offset)
}

#[php_function]
pub fn varint_read_bool_hex(hex: String, offset: Option<u64>) -> Result<Vec<u64>, PhpException> {
    VarInt::read_bool_hex(hex, offset)
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    macro_rules! test_fn {
        ($pack_fn:ident, $read_fn:ident, $tests:expr) => {
            for (input, expected) in $tests {
                assert_eq!(
                    VarInt::$pack_fn(input),
                    expected
                );

                if let Ok(result) = VarInt::$read_fn(expected.into(), Some(0), Some(0)) {
                    assert_eq!(result[0], input);
                } else {
                    panic!("error");
                }
            }
        };
    }
    #[test]
    pub fn test_pack_and_unpack_unsigned_ints() -> () {
        let tests = HashMap::from([
            ("0", "00"),
            ("35442", "f29402"),
            ("523472346", "da9bcef901"),
            ("23425234862394", "ba82b6e6e1a905"),
            ("7342523486232352394", "8aaddebed5c6f8f265"),
        ]);

        test_fn!(pack_uint_hex, read_uint_hex, tests);
    }


    #[test]
    pub fn test_pack_and_unpack_zero_signed_int() -> () {
        let tests = HashMap::from([
            (0, "00"),
            (35442, "e4a904"),
            (-523472346, "b3b79cf303"),
            (-34235, "f59604"),
            (-5234723234233423446, "abd9bc9fffc2bca59101"),
        ]);

        test_fn!(pack_int_hex, read_int_hex, tests);
    }
}
