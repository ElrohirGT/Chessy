<template>
    <main class="root">
        <div class="left-container"></div>
        <div class="chessboard"></div>
        <div class="right-container"></div>
    </main>
</template>

<script setup></script>

<style scoped>
.root {
    width: 100vw;
    max-width: 100vw;
    height: 100vh;
    background-image: url('../assets/Game/background.webp');

    display: flex;
    justify-content: center;
    align-items: center;
}

.chessboard {
    width: 600px;
    height: 600px;
    background-color: black;
    padding: 2ch;

    display: grid;
}
</style>
