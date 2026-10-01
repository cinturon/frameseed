use std::path::Path;

pub struct AudioAnalysis {
    pub beat_times: Vec<f32>,
    pub bpm: f32,
    pub duration_seconds: f32,
}

pub fn analyze_wav(path: &Path) -> Result<AudioAnalysis, String> {
    let mut reader = hound::WavReader::open(path).map_err(|e| e.to_string())?;
    let spec = reader.spec();
    let sample_rate = spec.sample_rate as f32;
    let channels = spec.channels as usize;

    // Decode all samples as f32 mono
    let mono: Vec<f32> = match spec.sample_format {
        hound::SampleFormat::Float => {
            let raw: Vec<f32> = reader.samples::<f32>().filter_map(|s| s.ok()).collect();
            to_mono(raw, channels)
        }
        hound::SampleFormat::Int => {
            let max = (1i64 << (spec.bits_per_sample - 1)) as f32;
            let raw: Vec<f32> = reader
                .samples::<i32>()
                .filter_map(|s| s.ok())
                .map(|s| s as f32 / max)
                .collect();
            to_mono(raw, channels)
        }
    };

    let duration_seconds = mono.len() as f32 / sample_rate;

    // RMS energy in 20 ms windows, 5 ms hop
    let window_len = ((sample_rate * 0.020) as usize).max(1);
    let hop_len = ((sample_rate * 0.005) as usize).max(1);

    let mut novelty: Vec<f32> = Vec::new();
    let mut prev_rms = 0.0f32;

    let mut pos = 0usize;
    while pos + window_len <= mono.len() {
        let chunk = &mono[pos..pos + window_len];
        let rms = (chunk.iter().map(|x| x * x).sum::<f32>() / window_len as f32).sqrt();
        novelty.push((rms - prev_rms).max(0.0));
        prev_rms = rms;
        pos += hop_len;
    }

    // Adaptive threshold: mean + 1.5 × std
    let n = novelty.len() as f32;
    let mean = novelty.iter().sum::<f32>() / n;
    let var = novelty.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / n;
    let threshold = mean + 1.5 * var.sqrt();

    // Peak picking with 100 ms minimum spacing
    let min_gap = ((0.1 / (hop_len as f32 / sample_rate)) as usize).max(1);
    let mut beat_times: Vec<f32> = Vec::new();
    let mut last = 0usize;

    for (i, &nov) in novelty.iter().enumerate() {
        if nov <= threshold {
            continue;
        }
        let is_local_max =
            (i == 0 || novelty[i - 1] <= nov) && (i + 1 >= novelty.len() || novelty[i + 1] <= nov);
        if is_local_max && i.saturating_sub(last) >= min_gap {
            beat_times.push(i as f32 * hop_len as f32 / sample_rate);
            last = i;
        }
    }

    // BPM from median inter-beat interval
    let bpm = if beat_times.len() >= 2 {
        let mut intervals: Vec<f32> = beat_times.windows(2).map(|w| w[1] - w[0]).collect();
        intervals.sort_by(f32::total_cmp);
        let median = intervals[intervals.len() / 2];
        if median > 0.0 { 60.0 / median } else { 0.0 }
    } else {
        0.0
    };

    Ok(AudioAnalysis {
        beat_times,
        bpm,
        duration_seconds,
    })
}

fn to_mono(samples: Vec<f32>, channels: usize) -> Vec<f32> {
    if channels == 1 {
        return samples;
    }
    samples
        .chunks(channels)
        .map(|ch| ch.iter().sum::<f32>() / channels as f32)
        .collect()
}
