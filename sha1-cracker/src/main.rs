use sha1::Digest;
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

const SHA1_HEX_STRING_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        println!("Usage:");
        println!("sha1_cracker: <wordlist.txt> <sha1_hash>");
        return Ok(());
}
// first implimentation of the RAH: Rusts Aquisition as an Initializier
// we do not have to close the file in rust because the file will close when the main cloes
// ll memory resources are init'd one time , it owns the range in memory and the space it requires for the entire  
// life cycle of program, no memory leaks and you pay costs upfront

let hash_to_crack = args[2].trim();

if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
    return Err("sha1 hash is not valid".into());
}

let wordlist_file = File::open(&args[1])?;
let reader = BufReader::new(&wordlist_file);
    
for line in reader.lines() {
    let line = line?;
    let common_password = line.trim();
    if hash_to_crack ==
    ↪
        &hex::encode(sha1::Sha1::digest(common_password.as_bytes())) {
        println!("Password found: {}", &common_password);
        return Ok(()); // the exit of a successfull Rust program is return Ok(())
        }
    }

    println!("password not found in wordlist :(");  
    Ok(()) // unsuccessful program is expressed as Ok(())

}
