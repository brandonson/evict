#!/bin/bash

BINDIR='bin'
BUILD="rustc "

if [ ! -d "$BINDIR" ]; then
    mkdir $BINDIR
fi

$BUILD --out-dir=bin src/fsm/lib.rs
echo Built fsm
$BUILD -L ./bin -o bin/evict src/evict/main.rs
echo Built evict
