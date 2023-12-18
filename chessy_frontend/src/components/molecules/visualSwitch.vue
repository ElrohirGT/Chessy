<template>
    <div class="container">
        <span
            class="font-button"
            :style="{ color: currentValue ? COLOR.PINK_COLOR : COLOR.WHITE_COLOR }"
            >{{ label }}</span
        >
        <label ref="switchButton" class="switch">
            <input type="checkbox" v-model="currentValue" />
            <span class="slider"></span>
        </label>
    </div>
</template>

<script setup>
import { ref, onMounted, watch, computed } from 'vue'
import { COLOR } from '../Color'
import SoundPlayer from '../../model/SoundPlayer'

/* VARIABLES */
/** The source of true, dictates the real stored value */
const currentValue = ref(false)

/* HTML ELEMENTS */
const switchButton = ref(null)

/** SOUND PLAYER */
let switchSound

/** EVENTS */
const emit = defineEmits(['switchUpdated'])

onMounted(() => {
    currentValue.value = props.initialValue
    console.log(switchButton)

    switchSound = new SoundPlayer(switchButton.value)
    switchSound.registerSoundOnEvent('click', '/sounds/selection2.mp3')
    switchSound.registerSoundOnEvent('dblclick', '/sounds/selection2.mp3')
})

const props = defineProps({
    label: {
        required: false,
        type: String,
        default: 'music'
    },
    initialValue: {
        required: true,
        type: Boolean,
        default: false
    },
    onText: {
        required: false,
        type: String,
        default: 'ON'
    },
    offText: {
        required: false,
        type: String,
        default: 'OFF'
    }
})

const onDisplayText = computed(() => {
    return `"${props.onText}"`
})

const offDisplayText = computed(() => {
    return `"${props.offText}"`
})

/**
 * Watches for changes in currentValue (Source of truth). And emits the
 * the signal to parent component the input value has changed.
 */
watch(currentValue, (newVal) => {
    emit('switchUpdated', newVal)
})
</script>

<style scoped>
.container {
    display: flex;
    align-items: center;
    justify-content: center;
}
.container span {
    display: inline-block;
    margin-right: 2ch;
}
.switch {
    position: relative;
    display: inline-block;
    min-width: 90px;
    height: 40px;
    outline: none;
}

.switch input {
    opacity: 0;
    width: 0;
    height: 0;
}

.slider {
    font-family: 'Luckiest Guy', cursive;
    position: absolute;
    z-index: 100;
    cursor: pointer;
    min-width: 90px;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: #000000;
    border: 3px solid #ffffff;
    -webkit-transition: 0.1s;
    transition: 0.1s;
}

.slider:before {
    content: v-bind(offDisplayText);
    position: absolute;
    left: 45px;
    bottom: -9px;
    z-index: 100;
    color: #000000;
    font-size: 2.5em;
    text-shadow: -2px -2px 0 #ffffff, 2px -2px 0 #ffffff, -2px 2px 0 #ffffff, 2px 2px 0 #ffffff;
    -webkit-transform: translateX(0px) rotate(-10deg);
    transform: translateX(0px) rotate(-10deg);
    -webkit-transition: 0.1s;
    transition: 0.1s;
}

.slider:after {
    content: '';
    position: absolute;
    top: -27px;
    left: 27px;
    width: 0;
    height: 0;
    border-top: 70px solid transparent;
    border-right: 37px solid #ffffff;
    border-bottom: 15px solid transparent;
    transform: rotate(90deg);
}

input:checked + .slider {
    border-color: #ff1ead;
    animation: shake 0.15s linear 0s 1;
}

input:checked + .slider:before {
    content: v-bind(onDisplayText);
    text-shadow: -2px -2px 0 #ff1ead, 2px -2px 0 #ff1ead, -2px 2px 0 #ff1ead, 2px 2px 0 #ff1ead;
    -webkit-transform: translateX(-65px) rotate(10deg);
    transform: translateX(-65px) rotate(10deg);
}

input:checked + .slider:after {
    content: '';
    position: absolute;
    border: none;
    top: 1px;
    left: 0px;
    width: 60px;
    height: 30px;
    transform: rotate(10deg) skew(-20deg);
    background: #ff1ead;
    box-shadow: -6px 3px #ff1ead;
}

@keyframes shake {
    0% {
        transform: translateX(0px) rotate(0deg);
    }
    33% {
        transform: translateX(2.5px) rotate(5deg);
    }
    66% {
        transform: translateX(-2.5px) rotate(-5deg);
    }
    100% {
        transform: translateX(0px) rotate(0deg);
    }
}
</style>
