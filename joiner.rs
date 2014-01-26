use std::os;
use std::io::File;

fn main() {
    let args: ~[~str] = os::args();
    if args.len() != 3 {
        println!("Usage: {:s} <inputfile> <inputfile>", args[0]); 
    } else {
        let ref secret1 = args[1];
        let ref secret2 = args[2];
        let path1 = Path::new(secret1.clone());
        let path2 = Path::new(secret2.clone());
        let msg1 = File::open(&path1);
        let msg2 = File::open(&path2);

        match (msg1, msg2) {
            (Some(mut file1), Some(mut file2)) => {
                let bytes1: ~[u8] = file1.read_to_end();
                let bytes2: ~[u8] = file2.read_to_end();
                xor(bytes1, bytes2);
            },
            (_,_) => fail!("Error opening message files: {:s}, {:s}", *secret1, *secret2)
        }
    }
}

fn xor(a: &[u8], b: &[u8]) {
    for i in range(0, a.len()) {
        print!("{:c}",(a[i] ^ b[i]) as char);
    }
}