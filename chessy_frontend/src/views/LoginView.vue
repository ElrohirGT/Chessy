<template>
    <main class="root" @mousemove="parallax">
        <img
            class="ghost"
            v-for="ghost in slidingGhosts"
            :key="ghost.source"
            :src="ghost.source"
            :style="`transform: translateX(${ghost.X_offset}px) translateY(${ghost.Y_offset}px);`"
        />
        <div class="login-box">
            <div class="back"></div>
            <div class="front">
                <textInput></textInput>
                <button class="play-btn font-button" @click="handleButton">Lets Play!</button>
            </div>
            <textBanner class="warning-msg" v-if="errorMsg" :text="errorMsg"></textBanner>
            <div class="title font-large">Chessy</div>
        </div>
    </main>
</template>

<script setup>
import { ref } from 'vue'
import textBanner from '../components/molecules/textBanner.vue'
import textInput from '../components/molecules/textInput.vue'
import ghost1 from '../assets/Login/ghost1.webp'
import ghost2 from '../assets/Login/ghost2.webp'
import ghost3 from '../assets/Login/ghost3.webp'
import ghost4 from '../assets/Login/ghost4.webp'
import { useRouter } from 'vue-router'

const router = useRouter()

const errorMsg = ref('Something went spooky wrong!')

// REPRESENTATION OF SLIDING GHOST
const slidingGhosts = ref([
    { source: ghost1, displaceMultiplier: 8, X_offset: 0, Y_offset: 0 },
    { source: ghost2, displaceMultiplier: 4, X_offset: 0, Y_offset: 0 },
    { source: ghost4, displaceMultiplier: 2, X_offset: 0, Y_offset: 0 },
    { source: ghost3, displaceMultiplier: 15, X_offset: 0, Y_offset: 0 }
])

// HANDLE LETS PLAY! BUTTON
function handleButton(event){
    router.push('/lobby')
}

// GHOST ANIMATION FUNCTION
function parallax(event) {
    slidingGhosts.value.forEach(function (ghost) {
        let moving_value = ghost.displaceMultiplier
        ghost.X_offset = (event.clientX * moving_value) / 200
        ghost.Y_offset = (event.clientY * moving_value) / 200
    })
}
</script>

<style scoped>
.root {
    width: 100vw;
    max-width: 100vw;
    height: 100vh;
    background-image: url('../assets/Login/background.webp');
    background-position: center;

    display: flex;
    align-items: center;
    justify-content: center;

    position: relative;
    overflow: hidden;
}

.ghost {
    position: absolute;
    height: 100%;
    top: 0;
}

.login-box {
    min-width: 60ch;
    height: 40ch;
    position: relative;
}

.login-box > .front,
.login-box > .back {
    width: 100%;
    height: inherit;
    position: absolute;
}

.login-box > .front {
    background-color: var(--white);
    color: black;
    display: flex;
    align-items: center;
    justify-content: center;
}
.login-box > .back {
    background-color: var(--red);
    transform: rotate(3deg);
    left: 5%;
    bottom: -5%;
}
.login-box > .title {
    position: absolute;
    right: 50%;
    top: -15%;
    transform: translateX(50%);
    z-index: 50;
    padding: 0.2ch;

    -webkit-user-select: none; /* Safari */
    -ms-user-select: none; /* IE 10 and IE 11 */
    user-select: none; /* Standard syntax */
}
.title::after {
    content: '';
    position: absolute;
    width: 110%;
    background-color: var(--red);
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;
    z-index: -1;
    transform: skew(-15deg) translateX(-0.2ch);
}

.warning-msg {
    position: absolute;
    bottom: -40%;
    right: 50%;
    width: 70%;
    transform: translateX(50%);
}

.play-btn {
    color: black;
    position: absolute;
    z-index: 10;
    bottom: 1ch;
    right: 1ch;
    cursor: pointer;
}
    
.play-btn:hover{
    animation: wiggle  linear 1s normal forwards;
    animation-fill-mode: forwards;
}

@keyframes wiggle {
    0% {
        transform: rotate(0deg);
    } 
    20% {
        transform: rotate(10deg);
    }
    100% {
        transform: rotate(10deg);
    }
}

</style>
