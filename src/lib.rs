#![allow(unused, non_snake_case, non_camel_case_types)]
use std::ffi::CString;
use std::os::raw::*;
use std::time::Duration;

#[repr(C)]
#[derive(Clone)]
pub struct Wave {
    frameCount: c_uint,
    sampleRate: c_uint,
    sampleSize: c_uint,
    channels: c_uint,
    data: *mut c_void
}

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
pub struct AudioStream {
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

impl Wave {
    pub fn load(path: &str) -> Self {
        let file = CString::new(path).unwrap();
        unsafe {
            LoadWave(file.as_ptr())
        }
    }

    pub fn is_ready(&self) -> bool {
        unsafe {
            IsWaveReady(self.clone())
        }
    }

    pub fn export(&self, file_name: &str) {
        let file = CString::new(file_name).unwrap();
        unsafe {
            ExportWave(self.clone(), file.as_ptr());
        }
    }

    pub fn crop(&mut self, init_sample: i32, final_sample: i32) {
        unsafe {
            WaveCrop(self as *mut Wave, init_sample as c_int, final_sample as c_int);
        }
    }

    pub fn format(&mut self, sample_rate: i32, sample_size: i32, channels: i32) {
        unsafe {
            WaveFormat(self as *mut Wave, sample_rate as c_int, sample_size as c_int, channels as c_int);
        }
    }
}

impl Drop for Wave {
    fn drop(&mut self) {
        unsafe {
            UnloadWave(self.clone());
        }
    }
}

impl AudioDevice {
    pub fn new() -> Self {
        unsafe {
            InitAudioDevice();
        }
        AudioDevice
    }

    pub fn is_ready(&self) -> bool {
        unsafe {
            IsAudioDeviceReady()
        }
    }

    pub fn set_master_volume(&self, volume: f32) {
        unsafe {
            SetMasterVolume(volume as c_float);
        }
    }

    pub fn get_master_volume(&self) -> f32 {
        let volume  = unsafe {
            GetMasterVolume()
        };
        volume as f32
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

    pub fn load_from_wave(device: &AudioDevice, wave: &Wave) -> Self {
        unsafe {
            LoadSoundFromWave(wave.clone())
        }
    }

    pub fn is_ready(&self) -> bool {
        unsafe {
            IsSoundReady(self.clone())
        }
    }

    pub fn play(&self) {
        unsafe { PlaySound(self.clone()); }
    }

    pub fn stop(&self) {
        unsafe { StopSound(self.clone()); }
    }

    pub fn pause(&self) {
        unsafe { PauseSound(self.clone()); }
    }

    pub fn resume(&self) {
        unsafe { ResumeSound(self.clone()); }
    }

    pub fn is_playing(&self) -> bool {
        unsafe { IsSoundPlaying(self.clone()) }
    }

    pub fn set_voume(&self, volume: f32) {
        unsafe { SetSoundVolume(self.clone(), volume as c_float); }
    }

    pub fn set_pitch(&self, pitch: f32) {
        unsafe { SetSoundPitch(self.clone(), pitch as c_float); }
    }

    pub fn set_pan(&self, pan: f32) {
        unsafe { SetSoundPan(self.clone(), pan as c_float); }
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

    pub fn is_ready(&self) -> bool {
        unsafe {
            IsMusicReady(self.clone())
        }
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

    pub fn seek(&self, position: f32) {
        unsafe {SeekMusicStream(self.clone(), position as c_float);}
    }

    pub fn set_volume(&self, volume: f32) {
        unsafe {SetMusicVolume(self.clone(), volume as c_float);}
    }

    pub fn set_pitch(&self, pitch: f32) {
        unsafe { SetMusicPitch(self.clone(), pitch as c_float); }
    }

    pub fn set_pan(&self, pan: f32) {
        unsafe { SetMusicPan(self.clone(), pan as c_float); }
    }

    pub fn duration(&self) -> Duration {
        let dur = unsafe {
            GetMusicTimeLength(self.clone())
        };
        Duration::from_secs_f32(dur as f32)
    }

    pub fn time_played(&self) -> Duration {
        let dur = unsafe {
            GetMusicTimePlayed(self.clone())
        };
        Duration::from_secs_f32(dur as f32)
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
    fn IsAudioDeviceReady() -> bool;
    fn SetMasterVolume(volume: c_float);
    fn GetMasterVolume() -> c_float;

    fn LoadWave(fileName: *const c_char) -> Wave;
    fn IsWaveReady(wave: Wave) -> bool;
    fn LoadSound(fileName: *const c_char) -> Sound;
    fn LoadSoundFromWave(wave: Wave) -> Sound;
    fn IsSoundReady(sound: Sound) -> bool;
    fn UnloadWave(wave: Wave);
    fn UnloadSound(sound: Sound);
    fn ExportWave(wave: Wave, fileName: *const c_char) -> bool;

    fn PlaySound(sound: Sound);
    fn StopSound(sound: Sound);
    fn PauseSound(sound: Sound);
    fn ResumeSound(sound: Sound);
    fn IsSoundPlaying(sound: Sound) -> bool;
    fn SetSoundVolume(sound: Sound, volume: c_float);
    fn SetSoundPitch(sound: Sound, pitch: c_float);
    fn SetSoundPan(sound: Sound, pan: c_float);
    fn WaveCrop(wave: *mut Wave, initSample: c_int, finalSample: c_int);
    fn WaveFormat(wave: *mut Wave, sampleRate: c_int, sampleSize: c_int, channels: c_int);

    fn LoadMusicStream(fileName: *const c_char) -> Music;
    fn IsMusicReady(music: Music) -> bool;
    fn UnloadMusicStream(music: Music);
    fn PlayMusicStream(music: Music);
    fn IsMusicStreamPlaying(music: Music) -> bool;
    fn UpdateMusicStream(music: Music);
    fn StopMusicStream(music: Music);
    fn PauseMusicStream(music: Music);
    fn ResumeMusicStream(music: Music);
    fn SeekMusicStream(music: Music, position: c_float);
    fn SetMusicVolume(music: Music, volume: c_float);
    fn SetMusicPitch(music: Music, pitch: c_float);
    fn SetMusicPan(music: Music, pan: c_float);
    fn GetMusicTimeLength(music: Music) -> c_float;
    fn GetMusicTimePlayed(music: Music) -> c_float;
}