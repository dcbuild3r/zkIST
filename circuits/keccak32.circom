pragma circom 2.0.0;

include "keccak.circom";

component main = Keccak(4*8, 32*8);