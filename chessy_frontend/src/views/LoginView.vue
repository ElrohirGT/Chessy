<template>
    <main class="root">
        <div class="login-box">
            <div class="back"></div>
            <div class="front"></div>
            <textBanner class="warning-msg" v-if="errorMsg" :text="errorMsg"></textBanner>
            <div class="title font-large">Chessy</div>
        </div>
    </main>
</template>

<script setup>
import { ref } from 'vue'
import textBanner from '../components/molecules/textBanner.vue'

const errorMsg = ref('Something went spooky wrong!')
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

.warning-msg{
    position: absolute;
    bottom: -40%;
    right: 50%;
    width: 70%;
    transform: translateX(50%);
}
</style>
