#![cfg_attr(windows, feature(abi_vectorcall))]
use ext_php_rs::{prelude::*, binary::Binary};
use faster_hex::{hex_string, hex_decode_fallback};
use std::str;

#[php_class]
pub struct VarInt;

#[php_impl(rename_methods = "camelCase")]
impl VarInt {
    #[public]
    pub fn pack_uint(value: u64) -> Binary<u8> {
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
    pub fn pack_uint_hex(value: u64) -> String {
        hex_string(Self::pack_uint(value).as_slice())
    }

    #[public]
    pub fn pack_int(value: i64) -> Binary<u8> {
        let mut ux: u64 = (value as u64) << 1u64;
        if value < 0 {
            ux = !ux;
        }
        Self::pack_uint(ux)
    }

    #[public]
    pub fn pack_int_hex(value: i64) -> String {
        hex_string(Self::pack_int(value).as_slice())
    }

    #[public]
    pub fn read_uint(bin: Binary<u8>, offset: Option<u64>, max_len: Option<u64>) -> Vec<u64> {
        let offset = offset.unwrap_or(0);
        let max_len = max_len.unwrap_or(0);

        let mut x: u64 = 0;
        let mut s: u64 = 0;
        let mut i: u64 = offset;
        let max_i = max_len + offset;

        // let chars: Vec<char> = bin.chars().collect();
        for u in bin.iter() {
            let u = u.to_owned();
            if u < 0x80 {
                if max_len > 0 && (i > max_i || i == max_i && u > 1) {
                    panic!("error");
                }
                return vec![(x | (u as u64) << s) as u64, i + 1];
            }
            x = x | ((u & 0x7f) as u64) << s;
            s += 7;
            i += 1;
        }

        vec![0, 0]
    }

    #[public]
    pub fn read_uint_hex(hex: String, offset: Option<u64>, max_len: Option<u64>) -> Vec<u64> {
        let mut dst = Vec::with_capacity(hex.len() / 2);
        hex_decode_fallback(hex.as_bytes(), &mut dst);
        Self::read_uint(Binary::new(dst), offset, max_len)
    }

    #[public]
    pub fn read_int(bin: Binary<u8>, offset: Option<u64>, max_len: Option<u64>) -> Vec<i64> {
        let input = Self::read_uint(bin, offset, max_len);
        let (ux, offset) = match input[..] {
            [a, b] => (a, b),
            _ => unreachable!(),
        };

        let mut x = (ux >> 1) as i64;
        let b = ux & 1;
        if b != 0 {
            x = !x;
        }

        vec![x, offset as i64]
    }

    #[public]
    pub fn read_int_hex(hex: String, offset: Option<u64>, max_len: Option<u64>) -> Vec<i64> {
        let mut dst = Vec::with_capacity(hex.len() / 2);
        hex_decode_fallback(hex.as_bytes(), &mut dst);
        Self::read_int(Binary::new(dst), offset, max_len)
    }

    #[public]
    pub fn read_bool(bin: Binary<u8>, offset: Option<u64>) -> Vec<u64> {
        Self::read_uint(bin, offset, Some(0u64))
    }

    #[public]
    pub fn read_bool_hex(hex: String, offset: Option<u64>) -> Vec<u64> {
        let mut dst = Vec::with_capacity(hex.len() / 2);
        hex_decode_fallback(hex.as_bytes(), &mut dst);
        Self::read_bool(Binary::new(dst), offset)
    }
}

#[php_function]
pub fn varint_pack_uint(value: u64) -> Binary<u8> {
    VarInt::pack_uint(value)
}

#[php_function]
pub fn varint_pack_int(value: i64) -> Binary<u8> {
    VarInt::pack_int(value)
}

#[php_function]
pub fn varint_read_uint(bin: Binary<u8>, offset: Option<u64>, max_len: Option<u64>) -> Vec<u64> {
    VarInt::read_uint(bin, offset, max_len)
}

#[php_function]
pub fn varint_read_int(bin: Binary<u8>, offset: Option<u64>, max_len: Option<u64>) -> Vec<i64> {
    VarInt::read_int(bin, offset, max_len)
}

#[php_function]
pub fn varint_read_int_hex(hex: String, offset: Option<u64>, max_len: Option<u64>) -> Vec<i64> {
    VarInt::read_int_hex(hex, offset, max_len)
}

#[php_function]
pub fn varint_read_bool(bin: Binary<u8>, offset: Option<u64>) -> Vec<u64> {
    VarInt::read_bool(bin, offset)
}

#[php_function]
pub fn varint_read_bool_hex(hex: String, offset: Option<u64>) -> Vec<u64> {
    VarInt::read_bool_hex(hex, offset)
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_pack_zero_unsigned_int() -> () {
        assert_eq!(
            VarInt::pack_uint_hex(0),
            "00"
        );
    }

    #[test]
    pub fn test_pack_non_zero_unsigned_ints() -> () {
        assert_eq!(
            VarInt::pack_uint_hex(35442),
            "f29402"
        );

        assert_eq!(
            VarInt::pack_uint_hex(523472346),
            "da9bcef901"
        );

        assert_eq!(
            VarInt::pack_uint_hex(23425234862394),
            "ba82b6e6e1a905"
        );
    }


    #[test]
    pub fn test_pack_zero_signed_int() -> () {
        assert_eq!(
            VarInt::pack_int_hex(0),
            "00"
        );
    }

    #[test]
    pub fn test_pack_non_zero_signed_ints() -> () {
        assert_eq!(
            VarInt::pack_int_hex(35442),
            "e4a904"
        );

        assert_eq!(
            VarInt::pack_int_hex(-523472346),
            "b3b79cf303"
        );

        assert_eq!(
            VarInt::pack_int_hex(-34235),
            "f59604"
        );
    }
}
