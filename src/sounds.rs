use rand::Rng;
use rodio::source::{SineWave, Source};
use rodio::{dynamic_mixer, OutputStreamHandle, Sink, OutputStream};
use std::time::Duration;

use crate::squarewave;


pub fn random_sound(
    freq_choices: &[f32],
    good_amplification: f32,
    stream_handle: &OutputStreamHandle,
) -> Sink {
    let mut rng = rand::thread_rng();
    let num_tones = rng.gen_range(1..5);
    let mut tones = Vec::new();
    let mut durations = Vec::new();
    for _n in 1..num_tones {
        let osc_freq_idx = rng.gen_range(0..freq_choices.len());
        let osc = SineWave::new(freq_choices[osc_freq_idx]);
        let duration = rng.gen_range(1..3);
        durations.push(duration);
        let mut sample = osc
            .amplify(good_amplification)
            .fade_in(Duration::from_secs_f32(0.5))
            .take_duration(Duration::from_secs(duration));
        sample.set_filter_fadeout();
        tones.push(sample);
    }
    let (controller, mixer) = dynamic_mixer::mixer::<f32>(2, 44_100);
    for tone in tones {
        controller.add(tone);
    }
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(mixer);
    sink
}

pub fn play_sounds() {
    // everything is at least half amplitude - generated sounds are very loud!
    let base_amp = 0.5;
    // the square wave is particularly obnoxious - reduce its amplitude even further
    let bad_base_amp = 0.05;
    let mut good_count = 80;
    let mut bad_count = 20;
    fn calc_pct(num: i32, den: i32) -> f32 {
        num as f32 / (num as f32 + den as f32)
    }
    let good_pct = calc_pct(good_count, bad_count);
    println!("good_pct{good_pct}");
    let bad_pct = calc_pct(bad_count, good_count);
    println!("bad_pct{bad_pct}");
    let mut good_amp = base_amp * good_pct;
    let mut bad_amp = base_amp * bad_base_amp * bad_pct;
    let pentatonic_frequencies = vec![440.0, 495.0, 556.875, 660.0, 742.5];

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let bad_tone = squarewave::SquareWave::new(440.0).amplify(bad_amp).repeat_infinite();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(bad_tone);
    // loop {
        random_sound(&pentatonic_frequencies, good_amp, &stream_handle).sleep_until_end();
        let mut rng = rand::thread_rng();
        let random_volume = rng.gen_range(0.0..1.0);
        sink.set_volume(random_volume);
    // }
}
