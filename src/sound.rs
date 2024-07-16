#![allow(unused)]
use std::ffi::CString;
use std::os::raw::*;

pub trait Audio {
    fn play(&self);
}
pub struct AudioManager;

impl AudioManager {
    pub fn play<T: Audio>(&self, audio: &T) {
        audio.play();
    }
}

impl Drop for AudioManager {
    fn drop(&mut self) {
        unsafe {
            CloseAudioDevice();
        }
    }
}

#[repr(C)]
#[allow(non_camel_case_types)]
struct rAudioBuffer {
    _private: [u8; 0]
}

#[repr(C)]
#[allow(non_camel_case_types)]
struct rAudioProcessor{
    _private: [u8; 0]
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Clone)]
struct AudioStream {
    buffer: *mut rAudioBuffer,
    processor: *mut rAudioProcessor,

    sampleRate: c_uint,
    sampleSize: c_uint,
    channels: c_uint
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Clone)]
pub struct Sound {
    stream: AudioStream,
    frameCount: c_uint
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Clone)]
pub struct Music {
    stream: AudioStream,
    frameCount: c_uint,
    looping: bool,

    ctxType: c_int,
    ctxData: *mut c_void
}

impl Drop for Sound {
    fn drop(&mut self) {
        unsafe {
            UnloadSound(self.clone());
        }
    }
}

impl Audio for Sound {
    fn play(&self) {
        unsafe { PlaySound(self.clone()); }
    }
}

impl Drop for Music {
    fn drop(&mut self) {
        unsafe {
            UnloadMusicStream(self.clone());
        }
    }
}

impl Audio for Music {
    fn play(&self) {
        unsafe { PlayMusicStream(self.clone()); }
    }
}

impl Music {
    pub fn is_playing(&self) -> bool {
        unsafe {IsMusicStreamPlaying(self.clone())}
    }

    pub fn update(&self) {
        unsafe {UpdateMusicStream(self.clone());}
    }

    pub fn stop(&self) {
        unsafe {StopMusicStream(self.clone());}
    }

    pub fn pause(&self) {
        unsafe {PauseMusicStream(self.clone());}
    }

    pub fn resume(&self) {
        unsafe {ResumeMusicStream(self.clone());}
    }

    pub fn set_volume(&self, volume: f32) {
        unsafe {SetMusicVolume(self.clone(), volume);}
    }
}

#[allow(non_snake_case)]
#[link(name="raylib")]
#[link(name="gdi32")]
#[link(name="winmm")]
extern {
    fn InitAudioDevice();
    fn CloseAudioDevice();
    fn LoadSound(fileName: *const c_char) -> Sound;
    fn UnloadSound(sound: Sound);
    fn PlaySound(sound: Sound);
    fn LoadMusicStream(fileName: *const c_char) -> Music;
    fn UnloadMusicStream(music: Music);
    fn PlayMusicStream(music: Music);
    fn IsMusicStreamPlaying(music: Music) -> bool;
    fn UpdateMusicStream(music: Music);
    fn StopMusicStream(music: Music);
    fn PauseMusicStream(music: Music);
    fn ResumeMusicStream(music: Music);
    fn SetMusicVolume(music: Music, volume: c_float);
}

pub fn init_audio_device() -> AudioManager{
    unsafe { InitAudioDevice(); }
    AudioManager
}

pub fn close_audio_device() {
    unsafe { CloseAudioDevice(); }
}

pub fn load_sound(path: &str) -> Sound {
    let file = CString::new(path).unwrap();
    unsafe { LoadSound(file.as_ptr()) }
}

pub fn load_music(path: &str) -> Music {
    let file = CString::new(path).unwrap();
    unsafe { LoadMusicStream(file.as_ptr()) }
}