use hound;
use std::env;

fn calculate_rms(samples: &[i16]) -> f64 {
    let sum_of_squares: f64 = samples.iter().map(|&sample| (sample as f64).powi(2)).sum();
    (sum_of_squares / samples.len() as f64).sqrt()
}

fn amplitude_to_db(amplitude: f64) -> f64 {
    20.0 * amplitude.log10()
}

fn filter_silence(samples: &[i16], window_size: usize, threshold_db: f64) -> Vec<i16> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < samples.len() {
        let end = (i + window_size).min(samples.len());
        let rms = calculate_rms(&samples[i..end]);
        let db = amplitude_to_db(rms);

        if db >= threshold_db {
            result.extend_from_slice(&samples[i..end]);
        }

        i += window_size;
    }

    result
}

fn save_wav(file_path: &str, samples: &[i16], spec: hound::WavSpec) {
    let mut writer = hound::WavWriter::create(file_path, spec).expect("Failed to create WAV file");
    for &sample in samples {
        writer.write_sample(sample).expect("Failed to write sample");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <input.wav> <output.wav> <threshold_db>", args[0]);
        return;
    }

    let input_file = &args[1];
    let output_file = &args[2];
    let threshold_db: f64 = args[3].parse().expect("Invalid threshold dB value");

    let mut reader = hound::WavReader::open(input_file).expect("Failed to open WAV file");
    let spec = reader.spec();
    let samples: Vec<i16> = reader.samples::<i16>().map(|s| s.unwrap()).collect();

    let window_size = (spec.sample_rate / 1000) as usize;  // 10ms windows
    let filtered_samples = filter_silence(&samples, window_size, threshold_db);
    println!("{:?}",&filtered_samples);
    save_wav(output_file, &filtered_samples, spec);
}