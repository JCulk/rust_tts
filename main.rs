use std::fmt::format;
use std::io;
use std::{thread, time};



use tts::*;
fn main() -> Result<(), Error> {
    env_logger::init();
    let mut tts = Tts::default()?;
    //the text in this line should be passed from our llm
    let mut words = "Some info from out llm modle";
    //This line speaks the text.
    tts.speak(format!("{}",words),false)?;
    println!("{}", words);
    let mut _input = String::new();
    io::stdin().read_line(&mut _input)?;

    Ok(()) // Explicitly return Ok(()) to match the expected return type
}
