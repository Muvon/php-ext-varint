# php-ext-varint

## Intro
This is port of [php-kiss-varint](https://github.com/Muvon/php-kiss-varint) package into `PHP` extension by using `Rust`.

The work is still in progress but it works! and benchmarks are great as we can see for now.

## Benchmarks

Here are benchmarks vs `php-kiss-varint` package

The numbers are avg for single call on 100000 runs

| test | time | perf increased |
|-|-|-|
|🐘 Composer package packUint & bin2hex | 0.000000489 | 1x |
|🦀 PHP extension packUintHex| 0.000000169 | 2.89x |
|🐘 Composer package packInt & bin2hex| 0.000003175 | 1x |
|🦀 PHP extension packIntHex| 0.000000175 | 18.14x |
|🐘 Composer package readUint & hex2bin| 0.000001576| 1x |
|🦀 PHP extension readUintHex| 0.000000216 | 7.29x |
|🐘Composer package readInt & hex2bin| 0.000003129 | 1x |
|🦀 PHP extension readIntHex| 0.000000245 | 12.77x |

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
