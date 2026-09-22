// For this program to work, Cargo.toml needs its rodio crate downgraded to "0.20.0" version.
// eframe, egui, and rodio - 3 libraries important for this project to work
use eframe::egui; 
use rodio::{Decoder, OutputStream,  OutputStreamHandle, Source};
use std::fs::File;
use std::io::BufReader;


// Our app's audio structure
struct SoundboardApp {
    _stream: OutputStream, // This is here to keep the audio in account, "_" is for to keep the audio within the App while its running.
    stream_handle: OutputStreamHandle, // "Remote controller" of audio playback - in charge of playing it, pausing it or stopping it. 
    
}



impl SoundboardApp {  //  Create a final audio output result of our sound implementation 
    fn new() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap(); // try_default is for your device's personal audio output to use for this program, followed by unwrap to crash the program in case there are any problems with this.
        SoundboardApp {_stream: stream, stream_handle}            
    }
}
// Tell eframe how our app works
impl eframe::App for SoundboardApp {
    // This function creates our GUI
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Create a central panel 
        egui::CentralPanel::default().show(ui, |ui| {
            
            ui.heading("WeCode Soundboard");
    
            
            ui.label("My first Rust GUI program.");


            // I got my sound files listed in 'path' of each button, you download and create your sounds, folders and their path.
            if ui.button("Meow").clicked(){
               play_sound("Sounds/meow.mp3", &self.stream_handle);
        
            }

            if ui.button("Woof").clicked(){
                play_sound("Sounds/woof.mp3", &self.stream_handle);
            }
           
           if ui.button("Growl").clicked(){
                play_sound("Sounds/growl.mp3", &self.stream_handle);
           }

           if ui.button("Squeak").clicked(){
                play_sound("Sounds/squeak.mp3", &self.stream_handle);
           }

            
        });
    }
}

fn play_sound(path: &str, stream_handle: &OutputStreamHandle) {
    if let Ok(file) = File::open(path) {
        if let Ok(source) = Decoder::new(BufReader::new(file)) {
            let converted_source = source.convert_samples::<f32>();   // Converts our sound to needed 32 bits by a decoder
            if let Err(err) = stream_handle.play_raw(converted_source) {
                eprintln!("Failed to play sound: {}", err);
            }
        } else {
            eprintln!("Could not decode file '{}'", path);   // If File is of unknown and/or unsupported origin
        }
    } else {
        eprintln!("Sound file not found: '{}'", path);
    }
}

// Start the app!
fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "WeCode Soundboard", // Window title
        options,
        Box::new(|_cc| Ok(Box::new(SoundboardApp::new()))), // Our app
    );
}
