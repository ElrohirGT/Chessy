class Sound {
    constructor(soundFilePath) {
        this.audio = new Audio(soundFilePath)
        this.isPlaying = false
        this.audio.addEventListener('ended', () => (this.isPlaying = false))
    }

    play(loop = false) {
        this.isPlaying = true
        this.audio.loop = loop
        this.audio.play()
    }
    stop() {
        this.isPlaying = false
        this.audio.pause()
    }
    reset() {
        this.isPlaying = false
        this.audio.pause()
        this.audio.loop = false
        this.audio.currentTime = 0
    }
}

export default class SoundPlayer {
    constructor(HTMLElement) {
        this.element = HTMLElement
        this.sounds = {}
    }

    registerSoundOnEvent(eventName, soundFilePath) {
        this.sounds[eventName] = new Sound(soundFilePath)
        this.element.addEventListener(eventName, () => this.playSound(eventName))
    }

    playSound(eventName, loop = false) {
        const sound = this.sounds[eventName]
        if (!sound.isPlaying) {
            sound.play(loop)
        }
    }

    stopSound(eventName) {
        const sound = this.sounds[eventName]
        if (sound) {
            sound.stop()
        }
    }

    resetSound(eventName) {
        const sound = this.sounds[eventName]
        if (sound) {
            sound.reset()
        }
    }
}
