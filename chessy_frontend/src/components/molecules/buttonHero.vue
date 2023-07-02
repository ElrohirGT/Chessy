<template>
    <button class="box">
        <!-- SPARKLES -->
        <svg class="sparkle" viewBox="0 0 96 96" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path
                d="M93.781 51.578C95 50.969 96 49.359 96 48c0-1.375-1-2.969-2.219-3.578 0 0-22.868-1.514-31.781-10.422-8.915-8.91-10.438-31.781-10.438-31.781C50.969 1 49.375 0 48 0s-2.969 1-3.594 2.219c0 0-1.5 22.87-10.406 31.781-8.908 8.913-31.781 10.422-31.781 10.422C1 45.031 0 46.625 0 48c0 1.359 1 2.969 2.219 3.578 0 0 22.873 1.51 31.781 10.422 8.906 8.911 10.406 31.781 10.406 31.781C45.031 95 46.625 96 48 96s2.969-1 3.562-2.219c0 0 1.523-22.871 10.438-31.781 8.913-8.908 31.781-10.422 31.781-10.422Z"
                fill="#fff"
            />
        </svg>
        <svg class="sparkle" viewBox="0 0 96 96" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path
                d="M93.781 51.578C95 50.969 96 49.359 96 48c0-1.375-1-2.969-2.219-3.578 0 0-22.868-1.514-31.781-10.422-8.915-8.91-10.438-31.781-10.438-31.781C50.969 1 49.375 0 48 0s-2.969 1-3.594 2.219c0 0-1.5 22.87-10.406 31.781-8.908 8.913-31.781 10.422-31.781 10.422C1 45.031 0 46.625 0 48c0 1.359 1 2.969 2.219 3.578 0 0 22.873 1.51 31.781 10.422 8.906 8.911 10.406 31.781 10.406 31.781C45.031 95 46.625 96 48 96s2.969-1 3.562-2.219c0 0 1.523-22.871 10.438-31.781 8.913-8.908 31.781-10.422 31.781-10.422Z"
                fill="#fff"
            />
        </svg>
        <svg class="sparkle" viewBox="0 0 96 96" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path
                d="M93.781 51.578C95 50.969 96 49.359 96 48c0-1.375-1-2.969-2.219-3.578 0 0-22.868-1.514-31.781-10.422-8.915-8.91-10.438-31.781-10.438-31.781C50.969 1 49.375 0 48 0s-2.969 1-3.594 2.219c0 0-1.5 22.87-10.406 31.781-8.908 8.913-31.781 10.422-31.781 10.422C1 45.031 0 46.625 0 48c0 1.359 1 2.969 2.219 3.578 0 0 22.873 1.51 31.781 10.422 8.906 8.911 10.406 31.781 10.406 31.781C45.031 95 46.625 96 48 96s2.969-1 3.562-2.219c0 0 1.523-22.871 10.438-31.781 8.913-8.908 31.781-10.422 31.781-10.422Z"
                fill="#fff"
            />
        </svg>
        <svg class="sparkle" viewBox="0 0 96 96" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path
                d="M93.781 51.578C95 50.969 96 49.359 96 48c0-1.375-1-2.969-2.219-3.578 0 0-22.868-1.514-31.781-10.422-8.915-8.91-10.438-31.781-10.438-31.781C50.969 1 49.375 0 48 0s-2.969 1-3.594 2.219c0 0-1.5 22.87-10.406 31.781-8.908 8.913-31.781 10.422-31.781 10.422C1 45.031 0 46.625 0 48c0 1.359 1 2.969 2.219 3.578 0 0 22.873 1.51 31.781 10.422 8.906 8.911 10.406 31.781 10.406 31.781C45.031 95 46.625 96 48 96s2.969-1 3.562-2.219c0 0 1.523-22.871 10.438-31.781 8.913-8.908 31.781-10.422 31.781-10.422Z"
                fill="#fff"
            />
        </svg>
        <svg class="sparkle" viewBox="0 0 96 96" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path
                d="M93.781 51.578C95 50.969 96 49.359 96 48c0-1.375-1-2.969-2.219-3.578 0 0-22.868-1.514-31.781-10.422-8.915-8.91-10.438-31.781-10.438-31.781C50.969 1 49.375 0 48 0s-2.969 1-3.594 2.219c0 0-1.5 22.87-10.406 31.781-8.908 8.913-31.781 10.422-31.781 10.422C1 45.031 0 46.625 0 48c0 1.359 1 2.969 2.219 3.578 0 0 22.873 1.51 31.781 10.422 8.906 8.911 10.406 31.781 10.406 31.781C45.031 95 46.625 96 48 96s2.969-1 3.562-2.219c0 0 1.523-22.871 10.438-31.781 8.913-8.908 31.781-10.422 31.781-10.422Z"
                fill="#fff"
            />
        </svg>
        <!-- TEXT -->
        <span class="glare">
            {{ props.text }}
        </span>
        <span class="text">
            {{ props.text }}
        </span>
        <span class="shadow" aria-hidden="true">
            {{ props.text }}
        </span>
    </button>
</template>

<script setup>
const props = defineProps({
    text: {
        type: String,
        required: true
    },
    backgroundColor: {
        type: String,
        required: false,
        default: 'var(--background)'
    },
    textColor: {
        type: String,
        required: false,
        default: 'var(--text)'
    },
    glare: {
        type: String,
        required: false,
        default: 'var(--primary-button)'
    },
    shadow: {
        type: String,
        required: false,
        default: 'var(--primary-button-dark)'
    }
})
</script>

<style scoped>
@import '@/assets/colors.css';

.box {
    position: relative;
    padding: 3ch;
    border-radius: 2ch;
    background-color: rgba(0, 0, 0, 0);
    transition: background-color 0.5s;
    --text: v-bind(textColor);
    --glare: v-bind(glare);
    --shadow: v-bind(shadow);
    --box: v-bind(backgroundColor);
    --font-size: 5rem;
}
.box * {
    font-family: 'Outfit', sans-serif;
    font-size: var(--font-size);
}

.sparkle {
    position: absolute;
    fill: white;
    z-index: 300;
    width: calc(var(--font-size) * 0.5);
    aspect-ratio: 1;
}
.box:hover {
    background-color: var(--box);
}
.shadow {
    position: relative;
    z-index: 10;
    color: var(--shadow);
}
.text {
    z-index: 100;
}
.glare {
    z-index: 200;
}

.text,
.glare {
    position: absolute;
    transform: translate(3px, -3px);
    transition: transform 0.3s, background-position 0s;
}

.glare {
    background-color: transparent;
    background-image: linear-gradient(
        -75deg,
        transparent 0%,
        transparent 5%,
        var(--glare) 5%,
        var(--glare) 10%,
        transparent 10%,
        transparent 25%,
        var(--glare) 25%,
        var(--glare) 40%,
        transparent 40%,
        transparent 100%
    );
    background-size: 200% 100%;
    background-position: 200%;
    -webkit-text-fill-color: transparent;
    -webkit-background-clip: text;
    background-clip: text;
}
.box:hover .text,
.box:hover .glare {
    background-position: 0%;
    transform: translate(8px, -8px);
    transition: transform 0.3s, background-position 1.2s;
}

.box:active .text,
.box:active .glare {
    transform: translate(3px, -3px);
    transition: translate 2s;
}

button:hover svg {
    animation: sparkle 0.75s calc((var(--delay-step) * var(--d)) * 2s) both;
}

@keyframes sparkle {
    50% {
        transform: translate(-50%, -50%) scale(var(--s, 1));
    }
}

button svg {
    --delay-step: 0.15;
    top: calc(var(--y, 50) * 1%);
    left: calc(var(--x, 0) * 1%);
    transform: translate(-50%, -50%) scale(0);
}

button svg:nth-of-type(1) {
    --x: 0;
    --y: 20;
    --s: 1.1;
    --d: 1;
}

button svg:nth-of-type(2) {
    --x: 15;
    --y: 80;
    --s: 1.25;
    --d: 2;
}

button svg:nth-of-type(3) {
    --x: 45;
    --y: 40;
    --s: 1.1;
    --d: 3;
}

button svg:nth-of-type(4) {
    --x: 75;
    --y: 60;
    --s: 0.9;
    --d: 2;
}

button svg:nth-of-type(5) {
    --x: 100;
    --y: 30;
    --s: 0.8;
    --d: 4;
}
</style>

l
