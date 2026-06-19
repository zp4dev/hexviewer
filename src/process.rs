use std::fs;
pub fn read(filename : String) -> Vec<u8>{
    let data = fs::read(filename).expect("Can't not read file");
    return data;
}
pub fn print_hex(data: &[u8],output: &str) {
    let mut result = String::new();
    for (i, chunk) in data.chunks(16).enumerate() {
        print!("{:08x}  ", i * 16);

        for b in chunk {
            print!("{:02x} ", b);
        }

        print!(" ");

        for b in chunk {
            let c = *b as char;
            if c.is_ascii_graphic() || c == ' ' {
                print!("\x1b[1m{}\x1b[0m", c);
                result.push(c);
            } else {
                print!("\x1b[1m.\x1b[0m");
                print!(".")
            }
        }

        println!();
        if output != "No" {
            fs::write(output,result.clone()).expect("Can't write output");
            println!("Wrote to {} successfully!",output);
        }
    }
}