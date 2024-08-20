<script setup>
import { ref, onMounted } from 'vue'
import { COLOR } from '../components/Color'
import textBanner from '../components/molecules/textBanner.vue'
import visualSwitch from '../components/molecules/visualSwitch.vue'
import SoundPlayer from '../model/SoundPlayer'

const WHITE_COLOR = COLOR.WHITE_COLOR
const BLACK_COLOR = COLOR.BLACK_COLOR
const GREY_COLOR = COLOR.GREY_COLOR
const RED_COLOR = COLOR.RED_COLOR

let isCreateSectionActive = ref(false)
let isJoinSectionActive = ref(false)
let isMusicOn = ref(false)

/** HTML ELEMENTS */
let musicSwitch = ref(null)
let createBtn = ref(null)
let joinBtn = ref(null)

/** SOUND PLAYERS */
let musicSwitchSound = ref(null)
let createBtnSound = ref(null)
let joinBtnSound = ref(null)

onMounted(() => {
    // SETTING SOUND EFFECTS.
    // Vue components are better fetched by document.getElementById than
    // using refs
    musicSwitch.value = document.getElementById('music-switch')
    musicSwitchSound.value = new SoundPlayer(musicSwitch.value)
    createBtnSound.value = new SoundPlayer(createBtn.value)
    joinBtnSound.value = new SoundPlayer(joinBtn.value)

    musicSwitchSound.value.registerSoundOnEvent('playBackgroundMusic', '/sounds/lobby-music.mp3')

    createBtnSound.value.registerSoundOnEvent('click', './sounds/selection4.mp3')
    createBtnSound.value.registerSoundOnEvent('mouseenter', './sounds/selection.mp3')
    createBtnSound.value.registerSoundOnEvent('mouseleave', './sounds/selection3.mp3')

    joinBtnSound.value.registerSoundOnEvent('click', './sounds/selection4.mp3')
    joinBtnSound.value.registerSoundOnEvent('mouseenter', './sounds/selection.mp3')
    joinBtnSound.value.registerSoundOnEvent('mouseleave', './sounds/selection3.mp3')
})

function tooggleMusic(eventValue) {
    if (eventValue) {
        isMusicOn.value = true
        musicSwitchSound.value.playSound('playBackgroundMusic', true)
    } else {
        isMusicOn.value = false
        musicSwitchSound.value.stopSound('playBackgroundMusic')
    }
}
</script>

<template>
    <main class="root">
        <textBanner class="username-banner" text="Smaugthur"></textBanner>
        <div class="music-control">
            <visualSwitch
                id="music-switch"
                ref="musicSwitch"
                :initial-value="isMusicOn"
                on-text="ON"
                off-text="OFF"
                @switch-updated="tooggleMusic"
            ></visualSwitch>
        </div>
        <div class="create-block">
            <button
                ref="createBtn"
                class="create-btn"
                @mouseenter="isCreateSectionActive = true"
                @mouseleave="isCreateSectionActive = false"
            >
                <svg
                    viewBox="0 0 679 233"
                    version="1.1"
                    xmlns="http://www.w3.org/2000/svg"
                    xmlns:xlink="http://www.w3.org/1999/xlink"
                    xml:space="preserve"
                    xmlns:serif="http://www.serif.com/"
                    style="
                        fill-rule: evenodd;
                        clip-rule: evenodd;
                        stroke-linejoin: round;
                        stroke-miterlimit: 2;
                    "
                >
                    <g transform="matrix(1,0,0,1,-231.947,-1498.87)">
                        <g transform="matrix(1,0,0,1,0,1161.93)">
                            <g
                                transform="matrix(1.02816,0.0207559,-0.0212021,1.05027,54.0546,-1232.15)"
                            >
                                <path
                                    d="M251.599,1522.27L208.006,1696.58L549.319,1704.1L600.042,1674.96L867.609,1679.88L860.078,1477.01L540.574,1495.44L458.99,1522.27L251.599,1522.27Z"
                                    style="fill: white"
                                />
                            </g>
                        </g>
                        <g transform="matrix(1,0,0,1,0,1161.93)">
                            <g
                                transform="matrix(0.927883,0.0187316,-0.0185756,0.92016,96.9725,-1024.73)"
                            >
                                <path
                                    d="M256.337,1554.41L207.059,1684.27L536.143,1693.01L598.395,1659.07L809.704,1658.35L886.288,1638.36L876.185,1493.7L532.751,1513.11L467.829,1539.34L256.337,1554.41Z"
                                    :fill="isCreateSectionActive ? RED_COLOR : BLACK_COLOR"
                                />
                            </g>
                        </g>
                    </g>
                </svg>
                <span>CREATE</span>
            </button>
            <div class="ghost-container" :class="{ 'zoom-in-right': isCreateSectionActive }">
                <svg
                    viewBox="0 0 1005 1135"
                    version="1.1"
                    xmlns="http://www.w3.org/2000/svg"
                    xmlns:xlink="http://www.w3.org/1999/xlink"
                    xml:space="preserve"
                    xmlns:serif="http://www.serif.com/"
                    style="
                        fill-rule: evenodd;
                        clip-rule: evenodd;
                        stroke-linejoin: round;
                        stroke-miterlimit: 2;
                    "
                >
                    <g transform="matrix(1,0,0,1,44.9809,-1161.93)">
                        <g transform="matrix(1,0,0,1,0,1161.93)">
                            <g
                                transform="matrix(0.985235,-1.16635e-16,2.73458e-16,1.05055,14.1741,-1220.66)"
                            >
                                <rect
                                    x="-60.042"
                                    y="1161.92"
                                    width="1020.04"
                                    height="1080"
                                    style="fill: none"
                                />
                                <clipPath id="_clip1">
                                    <rect x="-60.042" y="1161.92" width="1020.04" height="1080" />
                                </clipPath>
                                <g clip-path="url(#_clip1)">
                                    <g
                                        transform="matrix(1.01264,-0.0646996,0.0689889,0.94968,-142.369,78.0207)"
                                    >
                                        <path
                                            d="M255.602,1603.84L353.801,1666.97L388.383,1761.81L427.9,1800.19L485.527,1780.99L678.679,1937.09L570.324,2151.34L577.434,2173.38L547.181,2389.8L-115.763,2391.25L-115.763,1633.4L-61.449,1496.83L88.15,1409.38L254.789,1485.45L255.602,1603.84Z"
                                            style="fill: rgb(64, 64, 64)"
                                        />
                                    </g>
                                    <g
                                        transform="matrix(1.01264,-0.0646996,0.0689889,0.94968,-142.369,78.0207)"
                                    >
                                        <path
                                            d="M297.31,1580.89L388.302,1639.38L424.43,1738.46L438.811,1752.43L494.024,1734.02L731.305,1925.8L615.453,2154.87L620.225,2169.66L583.612,2431.59L-157.628,2433.21L-157.628,1625.38L-95.023,1467.97L85.774,1362.27L296.469,1458.46L297.31,1580.89ZM255.602,1603.84L254.789,1485.45L88.15,1409.38L-61.449,1496.83L-115.763,1633.4L-115.763,2391.25L547.181,2389.8L577.434,2173.38L570.324,2151.34L678.679,1937.09L485.527,1780.99L427.9,1800.19L388.383,1761.81L353.801,1666.97L255.602,1603.84Z"
                                            :fill="isCreateSectionActive ? WHITE_COLOR : GREY_COLOR"
                                        />
                                    </g>
                                    <g
                                        transform="matrix(1.01264,-0.0646996,0.0689889,0.94968,-134.636,47.4589)"
                                    >
                                        <path
                                            d="M295.056,1827.95L344.829,1825.62L408.061,1887.04L470.079,1866.37L582.994,1957.63L516.343,2089.42L472.394,2095.7L499.632,2180.13L480.943,2313.83L-39.645,2314.97L-39.645,1647.98L-0.406,1549.32L92.468,1495.02L179.006,1534.53L179.884,1662.28L205.74,1662.28L291.073,1717.13L320.689,1798.36L295.056,1827.95ZM446.444,2274.26L459.11,2183.65L439.624,2123.24L357.782,2101.81L401.626,2065.76L490.288,2053.09L533.158,1968.33L462.033,1910.84L397.728,1932.28L329.527,1866.02L267.171,1868.95L228.198,1844.59L275.455,1790.03L258.402,1743.26L194.098,1701.92L140.51,1701.92L139.536,1560.09L94.718,1539.63L31.388,1576.65L-0,1655.57L-0,2275.23L446.444,2274.26Z"
                                            :fill="isCreateSectionActive ? WHITE_COLOR : GREY_COLOR"
                                        />
                                    </g>
                                </g>
                            </g>
                        </g>
                    </g>
                </svg>
                <img src="../assets/Lobby/leftGhostActive.webp" alt="" />
                <img
                    src="../assets/Lobby/leftGhostInactive.webp"
                    alt=""
                    v-if="!isCreateSectionActive"
                />
            </div>
        </div>
        <div class="join-block">
            <button
                ref="joinBtn"
                class="join-btn"
                @mouseenter="isJoinSectionActive = true"
                @mouseleave="isJoinSectionActive = false"
            >
                <svg
                    viewBox="0 0 464 204"
                    version="1.1"
                    xmlns="http://www.w3.org/2000/svg"
                    xmlns:xlink="http://www.w3.org/1999/xlink"
                    xml:space="preserve"
                    xmlns:serif="http://www.serif.com/"
                    style="
                        fill-rule: evenodd;
                        clip-rule: evenodd;
                        stroke-linejoin: round;
                        stroke-miterlimit: 2;
                    "
                >
                    <g transform="matrix(1,0,0,1,-1133.15,-1498.03)">
                        <g transform="matrix(1,0,0,1,0,1161.93)">
                            <g transform="matrix(0.666647,0,0,0.829637,997.18,-900.201)">
                                <path
                                    d="M203.959,1553.59L304.161,1735.94L899.668,1681.32L849.977,1490.18L334.314,1507.07L203.959,1553.59Z"
                                    style="fill: white"
                                />
                            </g>
                        </g>
                        <g transform="matrix(1,0,0,1,0,1161.93)">
                            <g transform="matrix(0.666647,0,0,0.829637,999.557,-900.833)">
                                <path
                                    d="M267.623,1555.75L318.776,1691.18L849.977,1662.17L823.57,1510.14L362.907,1519.78L267.623,1555.75Z"
                                    :fill="isJoinSectionActive ? RED_COLOR : BLACK_COLOR"
                                />
                            </g>
                        </g>
                    </g>
                </svg>
                <span>JOIN</span>
            </button>
            <div class="ghost-container" :class="{ 'zoom-in-left': isJoinSectionActive }">
                <svg
                    viewBox="0 0 1018 1135"
                    xmlns="http://www.w3.org/2000/svg"
                    xml:space="preserve"
                    style="
                        fill-rule: evenodd;
                        clip-rule: evenodd;
                        stroke-linejoin: round;
                        stroke-miterlimit: 2;
                    "
                >
                    <g transform="translate(-960 -1161.93)">
                        <path style="fill: none" d="M960 1161.92h1017.96v1134.6H960z" />
                        <clipPath id="a"><path d="M960 1161.92h1017.96v1134.6H960z" /></clipPath>
                        <g clip-path="url(#a)">
                            <path
                                d="m1353.26 1715.78 31.53-32.74-10.45-21.95 71.28-41.22 6.28-6.52 2.68 1.34 105.52-61.03-59.46-89.06 185.29-61.67 74.41-68.26 190.77 41.64 103.99 221.76-69.44 61.54 39.37 90.01-.66 7.93 59.72 32.78-13.66 612.79-720.46-10.86-27.75-94.98-7.09-24.27-122.38-97.25 51.64-51.64-61.7-33.49-50.76-205.8 79.26-172.79 142.07 3.74z"
                                :fill="isJoinSectionActive ? WHITE_COLOR : GREY_COLOR"
                                transform="translate(29.733 9.911)"
                            />
                            <path
                                d="m1376.93 1773.51 76.39-79.34-4.78-10.04 189.41-109.54 3.47-1.92-51.76-77.53 125.88-41.9 61.56-56.48 134.29 29.31 73.9 157.59-63.76 56.51 9.34 36.65 36.07 82.45-2.54 30.48 61.84 33.95-11.62 521.48-621.67-9.37-28.1-96.22-86.48-68.71 49.85-49.85-37.62-36.78-58.52-31.76-39.95-161.98 55.24-120.41 129.56 3.41z"
                                style="fill: #404040"
                                transform="translate(29.733 9.911)"
                            />
                            <path
                                d="m1410.51 1817.22-.71-16.12 63.11-65.54 55.24 27.61 3.85-3.37-27.81-58.4 154.83-89.54 43.87-24.23-46.46-69.59 81.31-27.06 51.94-47.65 91.92 20.07 51.33 109.45-59.34 52.6 16.66 65.33 33.13 75.73-3.95 47.39 63.43 34.82-10.1 453.01-547.57-8.25-7.07-24.21-15.99-54.74-59.55-47.31 48.59-48.6-9.21-20.83-28.99-12.44-36.71-35.89-47.14-25.58-31.84-129.1 37.22-81.14 136.01 3.58z"
                                :fill="isJoinSectionActive ? WHITE_COLOR : GREY_COLOR"
                                transform="translate(29.733 9.911)"
                            />
                            <path
                                d="m1450.92 2259.69-19.03-65.16-38.05-30.24 38.05-38.06-24.1-54.55-35.49-15.22-35.55-34.76-38.06-20.66-25.37-102.86 22.83-49.78 96.42 2.54 6.34 36.89 48.21-15.33-2.54-58.25 35.52-36.89 53.28 26.64 40.6-35.52-25.37-53.29 127.23-73.58 48.24-26.64 31.38-10.15-45.73-68.5 45.73-15.23 44.25-40.59 58.11 12.68 33.32 71.05-55.82 49.47 22.5 88.22 30.78 70.36-5.07 60.9 64.7 35.52-8.88 398.35-488.43-7.36z"
                                style="fill: #404040"
                                transform="translate(29.733 9.911)"
                            />
                        </g>
                    </g>
                </svg>
                <img src="../assets/Lobby/rightGhostActive.webp" alt="" />
                <img
                    src="../assets/Lobby/rightGhostInactive.webp"
                    alt=""
                    v-if="!isJoinSectionActive"
                />
            </div>
        </div>
    </main>
</template>

<style scoped>
.root {
    width: 100vw;
    max-width: 100vw;
    height: 100vh;
    background-image: url('../assets/Lobby/LobbyBackground.webp');
    display: flex;
}

.username-banner {
    position: absolute;
    top: 4ch;
    left: 50%;
    transform: translateX(-50%);
    z-index: 100;
}
.music-control {
    position: absolute;
    right: 6ch;
    top: 1ch;
    transform: translateY(50%);
    z-index: 100;
    display: flex;
}
.music-control span {
    margin-right: 2ch;
}

.create-block,
.join-block {
    width: 50vw;
    height: 100vh;
    overflow: hidden;
    position: relative;

    display: flex;
    align-items: center;
    justify-content: center;
}

/*==================
    GHOST CONTAINER
====================*/
.ghost-container {
    width: 100%;
    height: inherit;
    z-index: 10;
    transition: 0.2s ease-in-out;
}
.ghost-container img,
.ghost-container svg {
    width: 85%;
    position: absolute;
}
.create-block .ghost-container img,
.create-block .ghost-container svg {
    bottom: -5%;
    left: -5%;
}
.join-block .ghost-container img,
.join-block .ghost-container svg {
    bottom: -5%;
    right: -5%;
}

/*==================
    BIG BUTTONS
====================*/
.create-btn,
.join-btn {
    width: fit-content;
    position: absolute;
    z-index: 40;

    display: flex;
    align-items: center;
    justify-content: center;
}

.create-btn:hover,
.join-btn:hover {
    animation: tiltBounce 0.15s ease;
    animation-fill-mode: forwards;
    cursor: pointer;
}
.create-btn:active,
.join-btn:active {
    animation: clickBounce 0.15s ease;
    animation-fill-mode: forwards;
    cursor: pointer;
}

.create-btn span,
.join-btn span {
    color: white;
    font-family: 'personaFontTitle';
    font-size: 5.5rem;
    z-index: 50;
    display: inline-block;

    -webkit-user-select: none; /* Safari */
    -ms-user-select: none; /* IE 10 and IE 11 */
    user-select: none; /* Standard syntax */
}
.create-btn span {
    transform: rotate(-3deg);
}
.create-btn svg {
    width: 125%;
    position: absolute;
    top: 40%;
    left: 50%;
    transform: translate(-50%, -50%);
}
.join-btn svg {
    width: 150%;
    position: absolute;
    top: 55%;
    left: 50%;
    transform: translate(-50%, -50%);
}

/*
.create-btn:hover + .ghost-container {
    animation: tiltBounce 0.15s ease;
    animation-fill-mode: forwards;
} */

/* ============== */
/* ANIMATIONS     */
/* ============== */

.zoom-in-left {
    transform: scale(1.05) translateX(-5%) translateY(-5%);
}
.zoom-in-right {
    transform: scale(1.05) translateX(5%) translateY(-5%);
}

@keyframes tiltBounce {
    0% {
        transform: scale(1) rotate(-7deg);
    }
    50% {
        transform: scale(1.3) rotate(-7deg);
    }
    100% {
        transform: scale(1) rotate(-7deg);
    }
}

@keyframes tiltBounceAlt {
    0% {
        transform: scale(1) rotate(7deg);
    }
    50% {
        transform: scale(1.3) rotate(7deg);
    }
    100% {
        transform: scale(1) rotate(7deg);
    }
}
@keyframes clickBounce {
    0% {
        transform: scale(1) rotate(-7deg);
    }
    100% {
        transform: scale(0.5) rotate(-7deg);
    }
}
</style>
