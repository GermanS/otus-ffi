#!/usr/bin/env perl

use strict;
use warnings;

use FFI::Raw;

my $shared = 'target/release/libotusffi.so';

my $packed = pack( 'LLI', 42, 21, !!1 );
my $arg = FFI::Raw::MemPtr -> new_from_buf( $packed, length $packed );

my $create_socket = FFI::Raw->new(
        $shared, 'create_socket',
        FFI::Raw::ptr, FFI::Raw::ptr
);

my $power = FFI::Raw -> new(
        $shared, 'get_power',
        FFI::Raw::uint, FFI::Raw::ptr
);

my $inc_power = FFI::Raw -> new(
        $shared, 'inc_power',
        FFI::Raw::uint, FFI::Raw::ptr
);

my $dec_power = FFI::Raw -> new(
        $shared, 'dec_power',
        FFI::Raw::uint, FFI::Raw::ptr
);

my $status = FFI::Raw -> new(
        $shared, 'status',
        FFI::Raw::uint, FFI::Raw::ptr
);

my $turn = FFI::Raw -> new(
        $shared, 'turn',
        FFI::Raw::uint, FFI::Raw::ptr
);

my $ptr = $create_socket -> ( $arg );

{
        printf "Initial state\n";
        printf "Current power [%s]\n", $power -> ( $ptr );
        printf "Current state [%s]\n", $status -> ( $ptr );
}

{
        printf "\nIncrease the power\n";

        my $nval = $inc_power -> ( $ptr );
        printf "New power [%s]\n", $nval;
}

{
        printf "\nDecrease the power twice\n";
        $dec_power -> ( $ptr );
        my $nval = $dec_power -> ( $ptr );
        printf "New power [%s]\n", $nval;
}

{
        printf "\nTurn off the device\n";
        my $state = $turn -> ( $ptr );
        printf "New state [%s]\n", $state;
}

