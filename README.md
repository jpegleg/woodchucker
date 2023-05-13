# woodchucker 🐿️

### u64 to hex and increment

Usage with demo output:
```
woodchucker 0
```
There is one required argument, which is a u64 (range 0 through 18446744073709551615).

A second argument is an option flag to only output the hex instead of Int,Hex,Binary,ByteArray,UTF-8/lossy,BitShift1,BinarySquare,Square,SquareRoot

This hex value is not the hex encoded string of the integer, but the hex encoding of the u64 as little-endian bytes. This is much more useful for fuzzing as the hex decode (via xxd) of this hex get's us raw binary.

```
nt: 333404064, Hex: "13DF57A0"
```
```
$ echo 13DF57A0 | xxd -r -p > t
$ cat -vET t
^SM-_WM- 

```
The STDOUT will start from the integer provided in the first argument, and increment up until the value reaches 18446744073709551615 at which point it loop around to 0 and keep going forever. 

The exception to this pattern is if the program is interrupted or killed in any way. An intentional disruption of STDOUT will result in a panic and exit. We can use this in a UNIX-style command line to pipe woodchucker STDOUT to other programs.
```
woodchucker 0 0 | head -n2000000 > woodchuck.dat
```
Example usage:
```
woodchucker 0 0 | head -n 30000000 | xxd -r -p | some-fuzzer-program blah blah
```
In this way we send in the first 30 million u64s as hex into xxd and then into the fuzzer which in turn submits them to the API or otherwise uses them to fuzz with. We might also generate input by first writing to a file, then extracting the columns or lines we want.

Another example that uses xxd to go from hex to raw and then sed to insert a newline for the fuzzer file after every 16 bytes:
```
woodchucker 0 0 | head -n 20000 | xxd -r -p | sed -e "s/.\{16\}/&\n/g" > fuzz.txt
```

#### Intentional Panics

Note that woodchucker will panic when the STDOUT is interrupted, like so:

```
thread 'main' panicked at 'failed printing to stdout: Broken pipe (os error 32)', library/std/src/io/stdio.rs:1008:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

There are also arithmatc panics on the full demo output. Using the hex only output (pass 2+ args, anything will do) will avoid those as they are caused by the math operations.

```
$ woodchucker 333333333333
Starting u64 int: 333333333333
thread 'main' panicked at 'attempt to multiply with overflow', /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/ops/arith.rs:345:45
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
So we would use the second argument to get that hex, example:

```
$ woodchucker 333333333333 0 | head -n3
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/woodchucker 333333333333 lksdfs`
Only outputting hex...
Starting u64 int: 333333333333
"4D9C370555"
thread 'main' panicked at 'failed printing to stdout: Broken pipe (os error 32)', library/std/src/io/stdio.rs:1008:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
Instead we have the panic from breaking STDIN with the head. If we drop the head, it will run "forever".

This is the way we cut off the woodchucker: use manipulation of STDOUT to trigger panic and termination. Alternatively, kill the program.

#### Use with xxd to get the raw binary from the hex.

#### This program is for educational and security research purposes.

## chaos test mode

Rather than using a fuzzer, a way to fuzz "directly" with woodchucker and xxd is to let the raw binary be processed by the terminal or systems.


Warning: this can cause crashes or strange behavior!

So we can "fuzz" a terminal (and eat up system resources!) by doing this:

```
woodchucker 0 0 | xxd -r -p
```

Executing the raw chaos like that is a chaos test we can unleash on systems to see how they handle it!
