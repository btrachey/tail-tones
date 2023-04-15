use linemux::MuxedLines;
use rand::Rng;
use rodio::source::{SineWave, Source};
use rodio::{dynamic_mixer, Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;
use std::io::{stdin, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::mpsc::{channel, Sender, TryRecvError, Receiver};
use std::sync::{atomic::AtomicBool, atomic::Ordering, Arc, Mutex};
use std::thread;
use std::time::Duration;

mod squarewave;

mod config;
use config::*;

mod sounds;
use sounds::*;

struct LogCounts {
    good_logs: u128,
    bad_logs: u128,
}

fn count_logs(receiver: &Receiver<String>, log_counter: &mut LogCounts) {
    // thread::spawn(move ||
    for line in receiver {
        log_counter.good_logs += 1;
    }
    // );
}

fn start_listener(sender: Sender<String>) {
    thread::spawn(move || {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => {
                sender.send(input).unwrap();
            }
            Err(e) => println!("an error!: {:?}", e),
        }
    });
}

#[tokio::main]
async fn tail_file(filenames: &Vec<String>, log_counter: &mut LogCounts) -> std::io::Result<()> {
    // let args: Vec<String>  = std::env::args().skip(1).collect();

    let mut lines = MuxedLines::new()?;

    // for f in args {
    for f in filenames {
        lines.add_file(&f).await?;
    }

    while let Ok(Some(line)) = lines.next_line().await {
        log_counter.good_logs += 1;
        println!("({}) {}", line.source().display(), line.line());
    }

    Ok(())
}

fn background_sounds(log_counter: &mut LogCounts) {
    thread::spawn(move || {
    loop {
        play_sounds();
        println!("good logs: {}", log_counter.good_logs);
    }});
}

fn main() {
   let mut log_counter = LogCounts {
       good_logs: 0,
       bad_logs: 0,
   };
    let args: Vec<String>  = std::env::args().skip(1).collect();
    tail_file(&args, &mut log_counter);
//    // loop {
//    //     let mut input = String::new();
//    //     match stdin().read_line(&mut input) {
//    //         Ok(len) => {
//    //             if len == 0 {
//    //                 return;
//    //             } else {
//    //                 println!("{}", input);
//    //             }
//    //         }
//    //         Err(error) => {
//    //             eprintln!("error: {}", error);
//    //             return;
//    //         }
//    //     }
//    // }
//        let (tx, rx) = channel();
//        start_listener(tx);
//        // for line in rx {
//        //     println!("got this back: {}", line);
//        // }
//        count_logs(&rx, &mut log_counter);
//        thread::sleep(Duration::from_secs(6));
//        println!("log count: {}", log_counter.good_logs);
//    // for line in rx {
//    //     println!("Got this back: {}", line);
//    // }
//    // thread::sleep(Duration::from_secs(2));

//    //

//    // let music_duration = Duration::from_secs_f32(5.);
//    // let (controller, mixer) = dynamic_mixer::mixer::<f32>(2, 44_100);
//    // let (_stream, handle) = OutputStream::try_default().unwrap();

//    // let sound_files =
//    // let config_dir_base_path = String::from("/.config/tail-tones/music/");
//    // let full_config_dir_value = match home_dir() {
//    //     Some(path) => format!("{}", path.display()) + &config_dir_base_path,
//    //     None => String::from("")
//    // };

//    //
//    // let music_dir_path = match config_music_dir_path() {
//    //     Some(path) => path,
//    //     None => PathBuf::new(),
//    // };

//    // let random_song = random_mp3(music_dir_path);
//    // print!("{}", random_song.unwrap().path().to_string_lossy());
//    // if random_song.is_ok() {
//    //     let song_path = random_song.unwrap().path();
//    //     let sink = Sink::try_new(&stream_handle).unwrap();
//    //     let music_source = Decoder::new(BufReader::new(File::open(song_path).unwrap())).unwrap();
//    //     sink.append(music_source);
//    //     sink.sleep_until_end();
//    // }
//    //

//    // let sink2 = Sink::try_new(&handle).unwrap();
//    // let beep_source = Decoder::new(BufReader::new(File::open("/Users/brian.tracey/Downloads/assets_beep.wav").unwrap())).unwrap().take_duration(music_duration);

//    // sink.append(beep_source);
//    // sink2.append(music_source);
//    // std::thread::sleep(Duration::from_millis(1000));
//    // drop(sink);

//    // sink2.sleep_until_end();
}
