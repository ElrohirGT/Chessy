<template>
    <div class="input-container" :class="{animate: isFocus}">
        <span class="label font-label-small">Username</span>
        <input ref="textInput" class="input font-label" type="text" @focusin="startJellyAnimation" @focusout="endJellyAnimation" />
    </div>
</template>

<script setup>
import { onMounted, ref } from 'vue';

const textInput = ref(null)
const isFocus = ref(false)
let animationInterval

onMounted(()=> {
    console.log(textInput);
})

function startJellyAnimation(){
    isFocus.value = true
    console.log("hey thee");
    animationInterval = setInterval(changeRandom, 750, textInput.value.parentNode)
}

function endJellyAnimation(){
    isFocus.value = false
    console.log("bye thee");
    window.clearInterval(animationInterval)
}

function changeRandom(varContainer) {
    for (let i = 1; i < 4; i++) {
        varContainer.style.setProperty('--random' + i, Math.floor(Math.random() * 6) - 3 + 'deg')
    }

    varContainer.style.setProperty('--random4', 1 + Math.random() / 20)
    varContainer.style.setProperty('--random5', 1 + Math.random() / 8)
}
</script>

<style scoped>
::-moz-selection {
    color: #fff;
    background: #18a8f0;
}
::selection {
    color: #fff;
    background: #18a8f0;
}

.input-container {
    position: relative;
    z-index: 10;
    --random1: 1deg;
    --random2: 2deg;
    --random3: 1.5deg;
    --random4: 1;
    --random5: 1;
    perspective: 300px;
}
.label {
    text-transform: uppercase;
    position: absolute;
    transform: translateY(-120%);
    top: 0px;
    left: 0;
    transition: 0.25s;
    color: var(--black);
}
.input {
    color: black;
    width: 100%;
    padding: 10px;
    outline: none;
    border: 3px solid #222;
    position: relative;
    z-index: 200;
}

.input-container:before,
.input-container:after {
    content: '';
    background: var(--red);
    position: absolute;
    z-index: -1;
    top: -5px;
    left: -5px;
    bottom: -5px;
    right: -5px;
    transform: scale(1.2);
    opacity: 0;
    transition: opacity 0.15s, transform 1s;
}

.input-container:after {
    top: -10px;
    left: -10px;
    bottom: -10px;
    right: -10px;
    background: var(--light-blue);
    z-index: -2;
}

.input-container.animate {
    transform: scale(1.2) skew(4deg, 2deg);
}

.input-container.animate .label {
    letter-spacing: 2px;
    font-weight: bold;
    top: -4px;
}
.input-container.animate:before,
.input-container.animate:after {
    opacity: 1;
    transform: scale(var(--random4), var(--random5)) skew(var(--random1), var(--random2));
}

.input-container.animate:after {
    transform: scale(var(--random4), var(--random5)) skew(var(--random3), var(--random1));
}
</style>
