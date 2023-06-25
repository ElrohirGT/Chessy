<template>
    <main>
        <h1>404</h1>
        <div class="container">
            <h3>Whoooops....</h3>
            <p>We're lost in space. The page you were trying to view does not exist :(</p>
            <routerLink to="/"> </routerLink>
            <button-important v-on:click="toHome()" class="home-button">
                <span>Back to Home </span>
                <i class="fa-solid fa-arrow-right"></i>
            </button-important>
        </div>
    </main>
</template>

<script>
import buttonImportant from '../components/molecules/buttonImportant.vue'

export default {
    components: {
        buttonImportant
    },
    methods: {
        toHome() {
            this.$router.replace({ path: '/' })
        }
    }
}
</script>

<style scoped>
@import '../assets/colors.css';

main {
    width: fit-content;
    position: absolute;
    left: 10vw;
    display: flex;
    flex-direction: row;
    align-items: center;
    top: 0px;
    bottom: 0px;
    padding: 3ch;
}

h1 {
    margin-right: 8rem;
    font-family: 'PT Serif', serif;
    font-size: 12rem;
    font-weight: bold;
}
h3 {
    font-size: 3.5rem;
}

.container {
    width: fit-content;
}
.home-button {
    margin-top: 5ch;
}

@media only screen and (max-width: 1480px) {
    main {
        position: static;
        width: 100%;
        height: 100vh;
        flex-direction: column;
        justify-content: center;
    }
    h1 {
        margin: 0rem;
    }

    .container {
        text-align: center;
    }
}
</style>
