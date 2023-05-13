fn nce(starter: u64) -> Vec<u8> {
    let mut sequence: Vec<u8> = Vec::new();
    let mut i: u64 = starter;
    loop {
        let bytes = i.to_le_bytes();
        let snork = format!("{:08b}", &i);
        let shark = format!("{:X}", &i);
        let mumbo = format!("{}", &i);
        let zeroo = &i >> 1;
        let multa = &i * &i;
        let morta = format!("{:08b}", &multa);
        let rawut = String::from_utf8_lossy(&bytes);
        let harka = (i as f64).sqrt();
        println!("Int: {}, Hex: {:?}, Binary: {:?}, LittleEndian byte array: {:?}, LittleEndian UTF-8/lossy: {:?}, Bit-Shift 1: {:?}, Squared binary: {:?}, Squared: {:?}, Square root: {:?} ", mumbo, shark, snork, bytes, rawut, zeroo, morta, multa, harka);
        sequence.extend(&bytes);
        i += 1;
    }
}

fn ncx(starter: u64) -> Vec<u8> {
    let mut sequence: Vec<u8> = Vec::new();
    let mut i: u64 = starter;
    loop {
        let bytes = i.to_le_bytes();
        let snork = format!("{:X}", &i);
        println!("{:?}", snork);
        sequence.extend(&bytes);
        i += 1;
    }
}

fn main() {
    if let Some(arg) = std::env::args().nth(2) {
        if let Ok(_stro) = arg.parse::<String>() {
            println!("Only outputting hex...");
            if let Some(arg) = std::env::args().nth(1) {
                if let Ok(num) = arg.parse::<u64>() {
                    println!("Starting u64 int: {}", &num);
                    ncx(num);
                } else {
                    println!("Invalid argument! Use a u64 (range 0 through 18446744073709551615)");
                }
             }
         }
     } else {         
        if let Some(arg) = std::env::args().nth(1) {
            if let Ok(num) = arg.parse::<u64>() {
                println!("Starting u64 int: {}", &num);
                nce(num);
            } else {
                println!("Invalid argument! Use a u64 (range 0 through 18446744073709551615)");
            }
        } else {
            println!("No argument provided.\n\nUsage:\n\nwoodchucker 0\n\nThere is one required argument, which is a u64 (range 0 through 18446744073709551615).\n\nA second argument is an option flag to only output the hex instead of Int,Hex,Binary,ByteArray,UTF-8/lossy,BitShift1,BinarySquare,Square,SquareRoot\n\nThe STDOUT will start from the integer provided in the first argument, and increment up until the value reaches 18446744073709551615 at which point it will loop around to 0 and continue. \n\nThe exception to this pattern is if the program is interrupted or killed in any way. An intentional disruption of STDOUT will result in a panic and exit. We can use this in a UNIX-style command line to pipe woodchucker STDOUT to other programs.\n\nwoodchucker 0 0 | head -n2000000 > woodchuck.dat\n\nExample usage:\n\nwoodchucker 0 0 | head -n 30000000 | xxd -r -p | some-fuzzer-program blah blah\n\nIn this way we send in the first 30 million u64s as hex into xxd and then into the fuzzer which in turn submits them to the API or otherwise uses them to fuzz with. We might also generate input by first writing to a file, then extracting the columns or lines we want.\n\nAnother example that uses xxd to go from hex to raw and then sed to insert a newline for the fuzzer file after every 16 bytes:\n\nwoodchucker 0 0 | head -n 20000 | xxd -r -p | sed -e \"s/.\\{{16\\}}/&\\n/g\" > fuzz.txt\n");

        }
    }
}
