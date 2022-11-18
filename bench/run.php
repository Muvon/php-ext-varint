<?php declare(strict_types=1);

include_once 'vendor/autoload.php';

function bench(string $test, callable $fn) {
  $time = [];
  $mem = [];
  for ($i = 0; $i <= 1000000; $i++) {
    $t = microtime(true);
    $m = memory_get_usage(true);
    $fn();
    $mem[] = memory_get_usage(true) - $m;
    $time[] = microtime(true) - $t;
  }

  $time = sprintf('%.9f', array_sum($time) / sizeof($time));
  // $memory = array_sum($mem) / sizeof($mem);
  echo "$test: $time\n";
}

bench('Composer package packUint (> PHP_INT_MAX)', function () {
  Muvon\KISS\VarInt::packUint("12223372036854775807");
});

bench('PHP extension packUint (> PHP_INT_MAX)', function () {
  Muvon\Ext\VarInt::packUint("12223372036854775807");
});

bench('Composer package packUintHex (> PHP_INT_MAX)', function () {
  bin2hex(Muvon\KISS\VarInt::packUint("12223372036854775807"));
});

bench('PHP extension packUint & bin2hex (> PHP_INT_MAX)', function () {
  Muvon\Ext\VarInt::packUintHex("12223372036854775807");
});


bench('Composer package packUint (= PHP_INT_MAX)', function () {
  Muvon\KISS\VarInt::packUint("9223372036854775807");
});

bench('PHP extension packUint (= PHP_INT_MAX)', function () {
  Muvon\Ext\VarInt::packUint("9223372036854775807");
});

bench('Composer package packUint & bin2hex (= PHP_INT_MAX)', function () {
  bin2hex(Muvon\KISS\VarInt::packUint("9223372036854775807"));
});

bench('PHP extension packUintHex (= PHP_INT_MAX)', function () {
  Muvon\Ext\VarInt::packUintHex("9223372036854775807");
});

bench('Composer package packInt – negative', function () {
  Muvon\KISS\VarInt::packInt(-7342523486232352394);
});

bench('PHP extension packInt – negative', function () {
  Muvon\Ext\VarInt::packInt(-7342523486232352394);
});

bench('Composer package packInt & bin2hex - negative', function () {
  bin2hex(Muvon\KISS\VarInt::packInt(-7342523486232352394));
});

bench('PHP extension packIntHex - negative', function () {
  Muvon\Ext\VarInt::packIntHex(-7342523486232352394);
});

bench('Composer package packInt – positive', function () {
  Muvon\KISS\VarInt::packInt(7342523486232352394);
});

bench('PHP extension packInt – positive', function () {
  Muvon\Ext\VarInt::packInt(7342523486232352394);
});

bench('Composer package packInt & bin2hex - positive', function () {
  bin2hex(Muvon\KISS\VarInt::packInt(7342523486232352394));
});

bench('PHP extension packIntHex - positive', function () {
  Muvon\Ext\VarInt::packIntHex(7342523486232352394);
});
bench('Composer package readUint & hex2bin', function () {
  Muvon\KISS\VarInt::readUint(hex2bin('8aad8e97b19694c120'));
});

bench('PHP extension readUintHex', function () {
  Muvon\Ext\VarInt::readUintHex('8aad8e97b19694c120');
});

bench('Composer package readInt & hex2bin', function () {
  Muvon\KISS\VarInt::readInt(hex2bin('93dabcfdaa8df1e5cb01'));
});

bench('PHP extension readIntHex', function () {
  Muvon\Ext\VarInt::readIntHex('93dabcfdaa8df1e5cb01');
});
