# woodchucker 🐿️

### u64 to hex and increment

Usage with demo output:
```
woodchucker 0
```
There is one required argument, which is a u64 (range 0 through 18446744073709551615).

A second argument is an option flag to only output the hex instead of Int,Hex,Binary,ByteArray,UTF-8/lossy,BitShift1,BinarySquare,Square,SquareRoot

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
This is the way we cut off the woodchucker: use manipulation of STDOUT to trigger panic and termination. Alternatively, kill the program.

#### Use with xxd to get the raw binary from the hex.

#### This program is for educational and security research purposes.
