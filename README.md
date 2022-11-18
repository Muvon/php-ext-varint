# php-ext-varint

## Intro

This is a port of [php-kiss-varint](https://github.com/Muvon/php-kiss-varint) package into the `PHP` extension by using `Rust`.

The work is still in progress but it works! and benchmarks are great as we can see for now.

## Benchmarks

Here are benchmarks vs `php-kiss-varint` package

The numbers are avg for a single call on 1 mln runs

| test | time | perf increased |
|-|-|-|
| 🐘 Composer package packUint (> PHP_INT_MAX) | 0.000001493 | 1x |
| 🦀 PHP extension packUint (> PHP_INT_MAX): |0.000000174 | 8.58x |
| 🐘 Composer package packUintHex (> PHP_INT_MAX) | 0.000001495 | 1x |
| 🦀 PHP extension packUint & bin2hex (> PHP_INT_MAX): | 0.000000188 | 7.95x |
| 🐘 Composer package packUint (= PHP_INT_MAX) | 0.000001157 | 1x |
| 🦀 PHP extension packUint (= PHP_INT_MAX): |0.000000181 | 6.39x |
| 🐘 Composer package packUint & bin2hex (= PHP_INT_MAX) | 0.000001200 | 1x |
| 🦀 PHP extension packUintHex (= PHP_INT_MAX): |0.000000192 | 6.25x |
| 🐘 Composer package packInt – negative | 0.000003072 | 1x |
| 🦀 PHP extension packInt – negative | 0.000000153 | 20.07x |
| 🐘 Composer package packInt & bin2hex - negative | 0.000003130 | 1x |
| 🦀 PHP extension packIntHex - negative | 0.000000167 | 18.74x |
| 🐘 Composer package packInt – positive | 0.000002064 | 1x |
| 🦀 PHP extension packInt – positive | 0.000000153 | 13.49x |
| 🐘 Composer package packInt & bin2hex - positive | 0.000002108 | 1x |
| 🦀 PHP extension packIntHex - positive | 0.000000168 | 12.54x |
| 🐘 Composer package readUint & hex2bin | 0.000001563 | 1x |
| 🦀 PHP extension readUintHex | 0.000000336 | 4.65x |
| 🐘 Composer package readInt & hex2bin | 0.000003037 | 1x |
| 🦀 PHP extension readIntHex | 0.000000229 | 13.26x |

## How to build

Just make sure that you have `rust` installed and run this command

```sh
cargo install cargo-php
cargo php install --release
```

Make sure that you have included it by using adding it to the `php.ini` file or a separate extension file

```ini
extension=libphp_ext_varint.so
```

You also can generate stubs to your IDE by running

```sh
cargo php stubs --stdout > stubs/VarInt.php
```
