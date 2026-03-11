use std::sync::Arc;

#[cfg(not(target_arch = "wasm32"))]
pub fn play_sound(data: Arc<Vec<i16>>, sample_rate: u32, channels: u16, volume: f32) {
    std::thread::spawn(move || {
        let Some(channel_count) = std::num::NonZeroU16::new(channels) else {
            return;
        };
        let Some(sample_rate) = std::num::NonZeroU32::new(sample_rate) else {
            return;
        };
        let samples: Vec<f32> = data
            .iter()
            .map(|sample| f32::from(*sample) / 32768.0)
            .collect();
        let stream = rodio::DeviceSinkBuilder::open_default_sink().unwrap();
        let sink = rodio::Player::connect_new(stream.mixer());
        let source = rodio::buffer::SamplesBuffer::new(channel_count, sample_rate, samples);
        sink.set_volume(volume);
        sink.append(source);
        sink.sleep_until_end();
    });
}

#[cfg(target_arch = "wasm32")]
pub fn play_sound(data: Arc<Vec<i16>>, sample_rate: u32, channels: u16, volume: f32) {
    use std::cell::RefCell;

    thread_local! {
        static AUDIO_STREAM: RefCell<Option<rodio::MixerDeviceSink>> = const { RefCell::new(None) };
    }

    let Some(channel_count) = std::num::NonZeroU16::new(channels) else {
        return;
    };
    let Some(sample_rate) = std::num::NonZeroU32::new(sample_rate) else {
        return;
    };
    let samples: Vec<f32> = data
        .iter()
        .map(|sample| f32::from(*sample) / 32768.0)
        .collect();

    AUDIO_STREAM.with(|stream| {
        let mut stream = stream.borrow_mut();
        if stream.is_none() {
            let Ok(new_stream) = rodio::DeviceSinkBuilder::open_default_sink() else {
                return;
            };
            *stream = Some(new_stream);
        }

        let Some(stream) = stream.as_ref() else {
            return;
        };

        let player = rodio::Player::connect_new(stream.mixer());
        let source = rodio::buffer::SamplesBuffer::new(channel_count, sample_rate, samples);
        player.set_volume(volume);
        player.append(source);
        player.detach();
    });
}
