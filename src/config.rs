use home::home_dir;
use rand::Rng;
use std::fs;
use std::io;
use std::path::PathBuf;

const CONFIG_BASE_PATH: &str = "/.config/tail-tones";

pub fn config_music_dir_path() -> Option<PathBuf> {
    let home_dir = home_dir()?;
    Some(PathBuf::from(format!(
        "{}{CONFIG_BASE_PATH}/music/",
        home_dir.display()
    )))
}

pub fn config_dir_path() -> Option<PathBuf> {
    let home_dir = home_dir()?;
    Some(PathBuf::from(format!(
        "{}{CONFIG_BASE_PATH}/",
        home_dir.display()
    )))
}

pub fn random_mp3(song_path: PathBuf) -> Result<fs::DirEntry, io::Error> {
    let mut rng = rand::thread_rng();
    let mut mp3_files = Vec::new();
    let mp3_dir_list = fs::read_dir(song_path)?;
    for path in mp3_dir_list {
        mp3_files.push(path)
    }
    let num_files = mp3_files.len();
    let rand_mp3_idx = rng.gen_range(0..num_files);
    mp3_files.remove(rand_mp3_idx)
}
