#![allow(unused, non_snake_case, non_camel_case_types)]
use std::ffi::CString;
use std::os::raw::*;

pub struct AudioDevice;

#[repr(C)]
struct rAudioBuffer {
    _private: [u8; 0]
}

#[repr(C)]
struct rAudioProcessor {
    _private: [u8; 0]
}

#[repr(C)]
#[derive(Clone)]
struct AudioStream {
    buffer: *mut rAudioBuffer,
    processor: *mut rAudioProcessor,

    sampleRate: c_uint,
    sampleSize: c_uint,
    channels: c_uint
}

#[repr(C)]
#[derive(Clone)]
pub struct Sound {
    stream: AudioStream,
    frameCount: c_uint
}

#[repr(C)]
#[derive(Clone)]
pub struct Music {
    stream: AudioStream,
    frameCount: c_uint,
    looping: bool,

    ctxType: c_int,
    ctxData: *mut c_void
}

impl AudioDevice {
    pub fn new() -> Self {
        unsafe {
            InitAudioDevice();
        }
        AudioDevice
    }
}

impl Drop for AudioDevice {
    fn drop(&mut self) {
        unsafe {
            CloseAudioDevice();
        }
    }
}

impl Sound {
    pub fn load(device: &AudioDevice, path: &str) -> Self {
        let file = CString::new(path).unwrap();
        unsafe { LoadSound(file.as_ptr()) }
    }

    pub fn play(&self) {
        unsafe { PlaySound(self.clone()); }
    }
}

impl Drop for Sound {
    fn drop(&mut self) {
        unsafe {
            UnloadSound(self.clone());
        }
    }
}

impl Music {
    pub fn load(device: &AudioDevice, path: &str) -> Self {
        let file = CString::new(path).unwrap();
        unsafe { LoadMusicStream(file.as_ptr()) }
    }

    pub fn play(&self) {
        unsafe { PlayMusicStream(self.clone()); }
    }

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
        unsafe {SetMusicVolume(self.clone(), volume as c_float);}
    }
}

impl Drop for Music {
    fn drop(&mut self) {
        unsafe {
            UnloadMusicStream(self.clone());
        }
    }
}

#[link(name = "audio")]
extern "C" {
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