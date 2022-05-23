pub struct Sound {}
pub struct Music {}

pub const CONTINUOUS: i32 = -1;
pub const DEFAULT_FADE_TIME: i64 = 500;

#[derive(PartialEq, PartialOrd)]
pub trait Mixer {
    // Mixer() = default;
    // Mixer(const Mixer&) = default;
    // Mixer& operator=(const Mixer&) = default;
    // Mixer(Mixer&&) = default;
    // Mixer& operator=(Mixer&&) = default;
    // virtual ~Mixer() = default;

    // virtual void playSound(const Sound& sound) = 0;
    fn playSound(sound: &Sound);
    // virtual void stopSound() = 0;
    fn stopSound();
    // virtual void pauseSound() = 0;
    fn pauseSound();
    // virtual void resumeSound() = 0;
    fn resumeSound();
    /**
     * Starts playing a Music track.
    	*
    	* \param music Reference to a Music Resource.
    	* \param loops Repeat count. -1 for continuous loop.
    	*/
    // void playMusic(const Music& music, int loops = Mixer::CONTINUOUS);
    // virtual void stopMusic() = 0;
    fn stopMusic();
    // virtual void pauseMusic() = 0;
    fn pauseMusic();
    // virtual void resumeMusic() = 0;
    fn resumeMusic();

    // virtual void fadeInMusic(const Music& music, int loops = Mixer::CONTINUOUS, std::chrono::milliseconds fadeInTime = Mixer::DEFAULT_FADE_TIME) = 0;
    fn fadeInMusic(music: &Music, loops: i32, fadeInTime: i32);

    // virtual void fadeOutMusic(std::chrono::milliseconds fadeOutTime = Mixer::DEFAULT_FADE_TIME) = 0;
    fn fadeOutMusic(fadeOutTime: i64);

    // virtual bool musicPlaying() const = 0;
    fn musicPlaying() -> bool;

    // void stopAllAudio();
    fn stopAllAudio();
    // void pauseAllAudio();
    fn pauseAllAudiO();
    // void resumeAllAudio();
    fn resumeAllAudio();

    /**
     * \param level Volume level, valid values are in the range [0, 128]
    	*/
    // virtual void soundVolume(int level) = 0;
    fn soundVolume(level: i32);

    /**
     * \param level Volume level, valid values are in the range [0, 128]
    	*/
    // virtual void musicVolume(int level) = 0;
    fn musicVolume(level: i32);

    /**
     * \return The volume level in the range [0, 128]
    	*/
    // virtual int soundVolume() const = 0;
    fn soundVolume2();

    /**
     * \return The volume level in the range [0, 128]
    	*/
    // virtual int musicVolume() const = 0;
    fn musicVolume2();

    // SignalSource<>& musicCompleteSignalSource();
    type mMusicCompleteSignalSource = SignalSource;

    fn playMusic(music: &Music, loops: i32 /*= Mixer::CONTINUOUS*/) {
        fadeInMusic(music, loops, 0);
    }

    fn stopAllAudio() {
        stopMusic();
        stopSound();
    }

    fn pauseAllAudio() {
        pauseMusic();
        pauseSound();
    }

    fn resumeAllAudio() {
        resumeMusic();
        resumeSound();
    }

    fn musicCompleteSignalSource(&self) -> SignalSource {
        return self.mMusicCompleteSignalSource;
    }
}
