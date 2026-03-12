use std::cell::RefCell;
use std::io::Cursor;
use std::sync::Arc;

thread_local! {
    static AUDIO_STREAM: RefCell<Option<rodio::MixerDeviceSink>> = const { RefCell::new(None) };
    static AUDIO_PLAYER: RefCell<Option<rodio::Player>> = const { RefCell::new(None) };
}

pub fn play_sound(data: Arc<[u8]>, volume: f32) {
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
        let Ok(source) = rodio::Decoder::new_wav(Cursor::new(data)) else {
            return;
        };

        AUDIO_PLAYER.with(|player| {
            let mut player_slot = player.borrow_mut();
            if let Some(active_player) = player_slot.take() {
                active_player.stop();
            }

            let new_player = rodio::Player::connect_new(stream.mixer());
            new_player.set_volume(volume);
            new_player.append(source);
            *player_slot = Some(new_player);
        });
    });
}

pub fn stop_audio_playback() {
    AUDIO_PLAYER.with(|player| {
        if let Some(active_player) = player.borrow_mut().take() {
            active_player.stop();
        }
    });
}

pub fn shutdown_audio_on_exit() {
    stop_audio_playback();

    AUDIO_STREAM.with(|stream| {
        if let Some(mut stream) = stream.borrow_mut().take() {
            // Normal app shutdown: this drop is expected and intentional.
            stream.log_on_drop(false);
            // Drop on explicit app shutdown instead of thread-local destructor teardown.
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                drop(stream);
            }));
        }
    });
}
