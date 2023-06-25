<template>
    <main>
        <div class="app-title">
            <i class="fa-regular fa-chess-knight"></i>
            <h1>Chessy</h1>
        </div>
        <form-card
            title="Login"
            :fields="this.fields"
            submitButtonText="Start Playing "
            submitButtonIcon="fa-solid fa-arrow-right"
            @form-submitted="fetchFieldsData()"
        ></form-card>
    </main>
</template>

<script>
import formCard from '@/components/organisms/formCard.vue'
import { REGISTER_URL } from '../main.js'

export default {
    components: {
        formCard
    },
    data() {
        return {
            fields: [
                {
                    id: 'username',
                    inputType: 'text',
                    placeholder: 'SuperChessGamer, jajaja',
                    currentValue: null
                },
                {
                    id: 'password',
                    inputType: 'password',
                    placeholder: 'Super secret he he he...',
                    currentValue: null
                }
            ]
        }
    },
    methods: {
        fetchFieldsData() {
            console.log('fetching datac')
            console.log(this.fields)
        },
        async registerUser() {
            let options = {
                method: 'POST',
                headers: { 'Content-Type': 'text/plain' },
                body: this.fields[0].currentValue
            }

            fetch(REGISTER_URL, options)
                .then((response) => response.text())
                .then((text) => console.log(text))
        }
    }
}
</script>

<style scoped>
@import '../assets/colors.css';
main {
    width: 100vw;
    height: 100vh;
    position: absolute;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 3ch;
}

.app-title {
    display: flex;
    align-items: center;
    margin-bottom: 3ch;
}
.app-title > img {
    height: 3rem;
}
.app-title > h1,
.app-title > i {
    font-size: 3rem;
    font-weight: bold;
}
.app-title > h1 {
    margin-left: 0.5ch;
}
</style>
