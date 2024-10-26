A very simple, high level, audio playback library, ported from [raudio](https://github.com/raysan5/raudio) library, for games.

## Supported audio formats
1. Can be played as `Sound` or `Music`: `.wav`, `.qoa`, `.ogg`, `.mp3`, `.flac`
2. Can only be played as `Music`: `.xm`

## Example
1. Example code of it being used in a standalone manner:

```rust
use radio_rs::*;

fn main() {
    let mut audio_device = AudioDevice::new();
    let music = Music::load("music.mp3");

    music.play();
    loop {
        music.update();
        audio_device.sync();
    }
}
```

2. Example code of it being used with [macroquad](https://github.com/not-fl3/macroquad):

```rust
use radio_rs::*;
use macroquad::prelude::*;

#[macroquad::main("sample")]
async fn main() {
    let audio_device = AudioDevice::new();
    let music = Music::load("music.mp3");

    music.play();
    loop {
        clear_background(LIGHTGRAY);
        music.update();
        audio_device.sync();
        
        next_frame().await
    }
}
```