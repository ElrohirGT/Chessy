<template>
    <main class="root">
        <div class="left-container"></div>
        <div class="chess-board"></div>
        <div class="right-container"></div>
    </main>
</template>

<script></script>

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

.root > * {
    height: 100px;
    width: 100px;
    background-color: red;
}
</style>
