A very simple, high level, audio playback library, ported from [raudio](https://github.com/raysan5/raudio) library, for games.

## Supported audio formats
1. Can be played as `Sound` or `Music`: `.wav`, `.qoa`, `.ogg`, `.mp3`, `.flac`
2. Can only be played as `Music`: `.xm`

## Example
Here is an example code of it being used with [macroquad](https://github.com/not-fl3/macroquad):

```rust
use radio_rs::*;
use macroquad::prelude::*;

#[macroquad::main("sample")]
async fn main() {
    let audio_device = AudioDevice::new();
    let music = Music::load(&audio_device, "music.mp3");

    music.play();
    loop {
        clear_background(LIGHTGRAY);
        music.update();
        
        next_frame().await
    }
}
```
