// ==================================================================================
// = NAS2D
// = Copyright © 2008 - 2020 New Age Software
// ==================================================================================
// = NAS2D is distributed under the terms of the zlib license. You are free to copy,
// = modify and distribute the software under the terms of the zlib license.
// =
// = Acknowledgment of your use of NAS2D is appreciated but is not required.
// ==================================================================================

// #include "MixerNull.h"

pub struct MixerNull {}

impl Mixer for MixerNull {
    fn playSound(sound: &Sound) {}

    fn stopSound() {}

    fn resumeSound() {}

    fn stopMusic() {}

    fn pauseMusic() {}

    fn resumeMusic() {}

    fn fadeInMusic(music: &Music, loops: i32, fadeInTime: i64) {}

    fn fadeOutMusic(fadeOutTIme: i64) {}

    fn musicVolume(level: i32) {}

    fn soundVolume2() -> i32 {
        0
    }

    fn musicVolume2() -> i32 {
        0
    }
}
