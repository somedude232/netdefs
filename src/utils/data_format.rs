
pub fn byte_dump(bytes: &[u8]) {
    for line in bytes.chunks(20) {
        for byte in line {
            print!("{:02X} ",byte);
        }
        println!("");
    };
}

