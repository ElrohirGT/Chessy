<template>
    <main>
        <div class="card create-room">
            <div>
                <button-hero
                    id="createRoomBtn"
                    text="Create room"
                    glare="var(--accent)"
                    shadow="var(--accent-dark)"
                ></button-hero>
            </div>
        </div>
        <div class="card join-room">
            <div>
                <button-hero id="joinRoomBtn" text="Join room"></button-hero>
            </div>
        </div>
    </main>
</template>

<script setup>
import buttonHero from '@/components/molecules/buttonHero.vue'
</script>

<style scoped>
@import '../assets/colors.css';

main {
    width: 100vw;
    height: 100vh;
    position: absolute;
    display: flex;
}

.create-room {
    box-sizing: content-box;
    border-right: 10px solid var(--text);
}

.create-room,
.join-room {
    position: relative;
}

.create-room::after,
.join-room::after {
    content: '';
    position: absolute;
    bottom: 0;
    left: 0;
    top: 0;
    right: 0;
    z-index: -1;
    opacity: 0;
    transition: opacity 1s ease-in-out, background-color 1s;
}

.create-room::after {
    background: #8cbeb2;
    background: linear-gradient(0deg, #8cbeb2 0%, rgba(242, 235, 191, 1) 100%);
}
.join-room::after {
    background: rgb(255, 168, 49);
    background: linear-gradient(0deg, rgba(255, 168, 49, 1) 0%, rgba(242, 235, 191, 1) 100%);
}

.create-room:hover::after,
.join-room:hover::after {
    opacity: 1;
}
.card {
    flex-grow: 1;
    flex-basis: 50vw;
    display: inline-flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    transition: flex-basis 1.5s ease-in-out;
}

.card:hover {
    flex-basis: 60vw;
}
</style>
