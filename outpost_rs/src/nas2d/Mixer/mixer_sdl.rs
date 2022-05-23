// ==================================================================================
// = NAS2D
// = Copyright © 2008 - 2020 New Age Software
// ==================================================================================
// = NAS2D is distributed under the terms of the zlib license. You are free to copy,
// = modify and distribute the software under the terms of the zlib license.
// =
// = Acknowledgment of your use of NAS2D is appreciated but is not required.
// ==================================================================================

// #include "MixerSDL.h"

// #include "../Resource/Sound.h"
// #include "../Resource/Music.h"

// #include "../configuration_h.rs"
// #include "../Utility.h"
// #include "../container_utils.rs"

// #include <SDL2/SDL.h>
// #include <SDL2/SDL_mixer.h>

// #include <array>
// #include <algorithm>
// #include <stdexcept>
// #include <string>


// using namespace NAS2D;



// ==================================================================================
// INTEROP WITH SDL2_MIXER
// ==================================================================================
// Global so it can be accessed without capturing `this`
// Signal<> musicFinished;
// ==================================================================================


// constexpr int AudioVolumeMin = 0;
pub const AudioVolumeMin: i32 = 0;
// constexpr int AudioVolumeMax = 128;
pub const AudioVolumeMax: i32 = 128;

// constexpr int AudioNumChannelsMin = 1;
pub const AudioNumChannelsMin: i32 = 1;
// constexpr int AudioNumChannelsMax = 2;
pub const AudioNumChannelsMax: i32 = 2;

// constexpr int AudioQualityLow = 11025;
pub const AudioQualityLow: i32 = 11025;
// constexpr int AudioQualityMedium = 22050;
pub const AudioQualityMedium: i32 = 22050;
// constexpr int AudioQualityHigh = 44100;
pub const AudioQualityHigh: i32 = 44100;

// constexpr auto AllowedMixRate = std::array{AudioQualityLow, AudioQualityMedium, AudioQualityHigh};
pub const AllowedMixRate: [i32; 3] = [
	AudioQualityLow, AudioQualityMedium, AudioQualityHigh
];

// constexpr int AudioBufferSizeMin = 256;
pub const AudioBufferSizeMin: i32 = 256;
// constexpr int AudioBufferSizeMax = 4096;
pub const AudioBufferSizeMax: i32 = 4096;


pub struct MixerSDL {

}

impl MixerSDL {
	fn InvalidToDefault(options: &Options) -> Options {
		// return {
	// 	has(AllowedMixRate, options.mixRate) ? options.mixRate : AudioQualityMedium,
	// 	std::clamp(options.numChannels, AudioNumChannelsMin, AudioNumChannelsMax),
	// 	std::clamp(options.sfxVolume, AudioVolumeMin, AudioVolumeMax),
	// 	std::clamp(options.musicVolume, AudioVolumeMin, AudioVolumeMax),
	// 	std::clamp(options.bufferSize, AudioBufferSizeMin, AudioBufferSizeMax)
	// };
		Options {
			
		}
	}

	fn ReadConfigurationOptions() -> Options {
		let configuration = Utility<Configuration>::get();
		let audio = configuration["audio"];
		// audio.get<int>("mixrate"),
		// audio.get<int>("channels"),
		// audio.get<int>("sfxvolume"),
		// audio.get<int>("musicvolume"),
		// audio.get<int>("bufferlength")
		Options {

		}
	}

	fn WriteConfigurationOptions(options: &Options)
	{
		// auto& configuration = Utility<Configuration>::get();
		let configuration = Utility<Configuration>::get();
		// auto& audio = configuration["audio"];
		let mut audio = configuration["audio"];
		// audio.set("mixrate", options.mixRate);
		audio.set("channels", options.numChannels);
		// audio.set("channels", options.numChannels);
		// audio.set("sfxvolume", options.sfxVolume);
		audio.set("sfxvolume", options.sfxVolume);
		// audio.set("musicvolume", options.musicVolume);
		audio.set("musicvolume", options.musicVolume);
		// audio.set("bufferlength", options.bufferSize);
		audio.set("bufferlength", options.bufferSize);
	}

	fn new() -> Self {
		let mut x = Self {};
		x.InvalidToDefault();
		x.ReadConfigurationOptions();
		x
	}

	fn new2(options: &Options) -> Self
	{
		let mut x = Self{};
		if (SDL_InitSubSystem(SDL_INIT_AUDIO) < 0)
		{
			// throw std::runtime_error(std::string{"Error initializing SDL audio: "} + SDL_GetError());
			panic!();
		}

		if (Mix_OpenAudio(options.mixRate, MIX_DEFAULT_FORMAT, options.numChannels, options.bufferSize))
		{
			// throw std::runtime_error(std::string{"Error opening audio mixer: "} + Mix_GetError());
			panic!();
		}

		x.soundVolume(options.sfxVolume);
		x.musicVolume(options.musicVolume);

		x.musicFinished.connect(this, &MixerSDL::onMusicFinished);
		// TODO:
		// Mix_HookMusicFinished([](){ musicFinished(); });
	}


}

impl Drop for  MixerSDL {
	fn drop(&mut self) {
		self.stopAllAudio();
		Mix_CloseAudio();
		Mix_HookMusicFinished();
		self.musicFinished.disconnect(self, &MixerSDL::onMusicFinished);
		SDL_QuitSubsystem(SDL_INIT_AUDIO)
	}
}

impl Mixer for MixerSDL {
	fn onMusicFinished(&mut self) {
		self.mMusicComplete.emit();
	}
}




void MixerSDL::playSound(const Sound& sound)
{
	Mix_PlayChannel(-1, sound.sound(), 0);
}


void MixerSDL::stopSound()
{
	Mix_HaltChannel(-1);
}


void MixerSDL::pauseSound()
{
	Mix_Pause(-1);
}


void MixerSDL::resumeSound()
{
	Mix_Resume(-1);
}


void MixerSDL::stopMusic()
{
	Mix_HaltMusic();
}


void MixerSDL::pauseMusic()
{
	Mix_PauseMusic();
}


void MixerSDL::resumeMusic()
{
	Mix_ResumeMusic();
}


void MixerSDL::fadeInMusic(const Music& music, int loops, std::chrono::milliseconds fadeInTime)
{
	Mix_FadeInMusic(music.music(), loops, static_cast<int>(fadeInTime.count()));
}


void MixerSDL::fadeOutMusic(std::chrono::milliseconds fadeOutTime)
{
	Mix_FadeOutMusic(static_cast<int>(fadeOutTime.count()));
}


bool MixerSDL::musicPlaying() const
{
	return Mix_PlayingMusic() == 1;
}


void MixerSDL::soundVolume(int volume)
{
	Mix_Volume(-1, std::clamp(volume, 0, SDL_MIX_MAXVOLUME));
}


void MixerSDL::musicVolume(int volume)
{
	Mix_VolumeMusic(std::clamp(volume, 0, SDL_MIX_MAXVOLUME));
}


int MixerSDL::soundVolume() const
{
	return Mix_Volume(-1, -1);
}


int MixerSDL::musicVolume() const
{
	return Mix_VolumeMusic(-1);
}
