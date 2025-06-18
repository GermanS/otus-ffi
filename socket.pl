#!/usr/bin/env perl

use strict;
use warnings;

use FFI::Raw;

my $shared = 'target/release/libotusffi.so';

my $packed = pack('LL', 42, 21);
my $arg = FFI::Raw::MemPtr->new_from_buf($packed, length $packed);

my $createCounter = FFI::Raw->new(
  $shared, 'createCounter',
  FFI::Raw::ptr, FFI::Raw::ptr
);

my $getCounterValue = FFI::Raw->new(
  $shared, 'getCounterValue',
  FFI::Raw::uint, FFI::Raw::ptr
);

my $incrementCounterBy = FFI::Raw->new(
  $shared, 'incrementCounterBy',
  FFI::Raw::uint, FFI::Raw::ptr
);

my $ptr = $createCounter->($arg);

my $val = $getCounterValue->($ptr);
print "Current value [$val]";

my $nval = $incrementCounterBy->($ptr);
print "New value [$nval]";