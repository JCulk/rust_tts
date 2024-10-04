use tts::Tts;

fn main() {
    // Initialize the TTS engine
    let mut tts = Tts::default().expect("Failed to initialize TTS engine");

    // The text you want to convert to speech
    let text = "This is my evidence for the week.";

    // Speak the text
    tts.speak(text,true).expect("Failed to speak");

    // Optional: You can also stop speaking if you need to interrupt
    // tts.stop().expect("Failed to stop TTS");
}

