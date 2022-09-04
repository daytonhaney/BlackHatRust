
use sha1::Digest; // calling crates Digest
use std::{ 
    env, // std env and error for errors, fs to talk to files like nodejs // and io for buffer communication 
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
}; 

const SHA1_HEX_STRING_LENGTH: usize = 40; // 

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        println!("Usage:");
        println!("sha1_cracker: <wordlist.txt> <sha1_hash>");
        return Ok(());
}
// first implimentation of the RAH: Rusts Aquisition as an Initializier
// in rust, we do not have to close the file that we create and open wordlist.txt because the file will close when the main cloes , un like C you 
// must manually close all files that are opened, else: buffer overflow; back door time.     
// all memory resources are init'd only one time , it owns the range in memory and space required for the entire  
// life cycle of program, no memory leaks , but you pay costs upfront with long devemopment process. 

let hash_to_crack = args[2].trim();

if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH { // error handling in rust is straight firward on *some stuff
    return Err("sha1 hash is not valid".into());
}

let wordlist_file = File::open(&args[1])?; // this is the file i am regading in the comments above 
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

    println!("password not found in wordlist :(");  // unsuccessful return Ok(()) != Ok(())
    Ok(()) // unsuccessful program is expressed as Ok(())

}
