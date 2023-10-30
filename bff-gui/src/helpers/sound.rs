use std::io::Cursor;
use std::sync::Arc;

pub fn play_sound(data: Arc<Vec<i16>>, sample_rate: u32, channels: u16, volume: f32) {
    std::thread::spawn(move || {
        let spec = hound::WavSpec {
            channels,
            sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut bytes = Vec::new();
        let mut write_cursor = Cursor::new(&mut bytes);
        let mut parent_writer = hound::WavWriter::new(&mut write_cursor, spec).unwrap();
        let mut sample_writer = parent_writer.get_i16_writer(data.len() as u32);

        for sample in data.iter() {
            sample_writer.write_sample(*sample);
        }
        sample_writer.flush().unwrap();
        parent_writer.finalize().unwrap();

        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&stream_handle).unwrap();
        let buf = std::io::BufReader::new(Cursor::new(bytes));
        let source = rodio::Decoder::new_wav(buf).unwrap();
        sink.set_volume(volume);
        sink.append(source);
        sink.sleep_until_end();
    });
}
