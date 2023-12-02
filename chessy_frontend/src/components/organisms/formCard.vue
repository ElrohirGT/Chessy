<template>
    <div class="formCard">
        <h1>{{ title }}</h1>
        <form>
            <!-- FIELDS  -->
            <div v-for="field in fields" :key="field.id">
                <div class="fieldName">
                    {{ formatFieldTitle(field.id) }}
                </div>
                <form-field
                    :inputType="field.inputType"
                    :placeholder="field.placeholder"
                    v-model="field.currentValue"
                ></form-field>
            </div>
            <!-- ERROR CARD -->
            <div class="submitError" v-if="errorMessage != null">
                <i class="fa-solid fa-circle-exclamation"></i>
                <span>{{ errorMessage }}</span>
            </div>
            <!-- SUBMIT BUTTON -->
            <button-important class="playButton" @click.prevent="submit()">
                {{ submitButtonText }}
                <i :class="submitButtonIcon" v-if="submitButtonIcon != null"></i
            ></button-important>
        </form>
    </div>
</template>

<script>
import formField from '@/components/molecules/formField.vue'
import buttonImportant from '@/components/molecules/buttonImportant.vue'

/**
 * A container for formFields components, so it represents a form component.
 */
export default {
    components: {
        formField,
        buttonImportant
    },
    props: {
        /** Form title */
        title: {
            type: String,
            required: true
        },
        /** Array of fields required in the form 
         * Example:
         * fields: [
                {
                    id: 'username',
                    inputType: 'text',
                    placeholder: 'SuperChessGamer, TheWinner...',
                    currentValue: null
                }
            ]
        */
        fields: {
            type: Array,
            required: true
        },
        /** Text to display in submit button */
        submitButtonText: {
            type: String,
            required: false,
            default: 'Submit'
        },
        /** Font Awesome code to display an icon at the end of button text */
        submitButtonIcon: {
            type: String,
            required: false,
            default: null
        },
        /** Message when form validation went wrong */
        errorMessage: {
            type: String,
            required: false,
            default: null
        }
    },
    emits: ['formSubmitted'],
    methods: {
        /**
         * Called when user submit its form.
         */
        submit() {
            this.$emit('formSubmitted')
        },
        /**
         * Capitalize the first letter of the string.
         * @param {String} text
         */
        formatFieldTitle(text) {
            return text.charAt(0).toUpperCase() + text.slice(1)
        }
    }
}
</script>

<style scoped>
@import '@/assets/color.css';
h1 {
    text-align: center;
}
form {
    display: flex;
    flex-direction: column;
    align-items: center;
}
.formCard {
    padding: 2ch;
    display: block;
    width: max-content;
    background-color: var(--accent);
    box-shadow: rgba(0, 0, 0, 0.16) 0px 10px 36px 0px, rgba(0, 0, 0, 0.06) 0px 0px 0px 1px;
    border-radius: 0.5rem;
    position: relative;
}
.fieldName {
    margin-bottom: 0.5rem;
    margin-top: 2ch;
    font-size: 1.2rem;
}
.playButton {
    display: block;
    margin-top: 3ch;
    margin-left: auto;
    margin-right: 0;
}
.submitError {
    margin-top: 1ch;
    background-color: rgb(241, 161, 161);
    padding: 1ch;
    border-radius: 0.5rem;
    outline: 2px solid rgb(180, 73, 73);
}
.submitError * {
    color: rgb(114, 37, 37);
}
.submitError i {
    margin-right: 1ch;
}
</style>
