<template>
    <main>
        <!-- APP LOGO -->
        <div class="app-title">
            <i class="fa-regular fa-chess-knight"></i>
            <h1>Chessy</h1>
        </div>
        <!-- LOGIN FORM -->
        <form-card
            title="Register"
            :fields="this.fields"
            :errorMessage="this.errorMsg"
            submitButtonText="Start Playing "
            submitButtonIcon="fa-solid fa-arrow-right"
            @form-submitted="signInUser"
        ></form-card>
    </main>
</template>

<script>
import { useVuelidate } from '@vuelidate/core'
import { required } from '@vuelidate/validators'
import { useUserSessionStore } from '@/stores/userSession'
import { REGISTER_URL } from '@/main.js'
import formCard from '@/components/organisms/formCard.vue'

export default {
    setup() {
        return { v$: useVuelidate() }
    },
    components: {
        formCard
    },
    data() {
        return {
            fields: [
                {
                    id: 'username',
                    inputType: 'text',
                    placeholder: 'SuperChessGamer, TheWinner...',
                    currentValue: null
                }
            ],
            errorMsg: null
        }
    },
    computed: {
        username() {
            return this.fields[0].currentValue
        }
    },
    validations() {
        return {
            username: { required }
        }
    },
    methods: {
        /**
         * Ask the API to create a user, with this.username,
         * and returns the id.
         */
        async registerUser() {
            let options = {
                method: 'POST',
                headers: { 'Content-Type': 'text/plain' },
                body: this.username
            }
            try {
                let response = await fetch(REGISTER_URL, options)
                let user_id = await response.text()
                console.log(this.username + ': ' + user_id)
                return user_id
            } catch (error) {
                this.errorMsg = 'An error happen in server, try again...'
                return null
            }
        },
        validateForm() {
            this.v$.$validate()
            if (this.v$.$error) {
                this.errorMsg = this.v$.$errors[0].$message
            }
            return this.v$.$error
        },
        /** Stores the user data from the form and change page. */
        async signInUser() {
            this.errorMsg = null
            if (this.validateForm()) return

            // Fetching values
            let user_id = await this.registerUser()
            if (this.errorMsg != null) return

            // Storing values
            const { setUserCredentials } = useUserSessionStore()
            setUserCredentials({
                name: this.username,
                id: user_id
            })

            this.$router.replace({ name: 'lobby' })
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
