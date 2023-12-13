<template>
    <div class="banner-container">
        <div class="frame"></div>
        <div class="back"></div>
        <span class="font-label">{{ text }}</span>
    </div>
</template>

<script setup>
const props = defineProps({
    text: {
        required: true,
        type: String,
        default: 'Text'
    }
})
</script>

<style scoped>
.banner-container {
    display: inline-block;
    position: relative;
    text-align: center;
    height: fit-content;
    z-index: 1;
}
.banner-container span {
    display: inline-block;
    position: relative;
    padding: 0.5ch 2ch;
    z-index: 5;
    transform: translate(0.5ch);
}
.frame,
.back {
    width: 100%;
    height: 100%;
    position: absolute;
    bottom: 0;
    left: 0;
    transform-origin: bottom left;
}

.frame {
    background-color: var(--black);
    border: 3px solid var(--white);
    transform: skew(-20deg);
    z-index: 2;
}

.back {
    background-color: var(--white);
    border: 3px solid var(--white);
    transform: skew(-20deg) translate(1ch, 1ch);
    z-index: 1;
}
</style>
