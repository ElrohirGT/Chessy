<template>
    <div class="formCard">
        <h1>{{ title }}</h1>
        <form>
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

export default {
    components: {
        formField,
        buttonImportant
    },
    props: {
        title: {
            type: String,
            required: true
        },
        fields: {
            type: Array,
            required: true
        },
        submitButtonText: {
            type: String,
            required: false,
            default: 'Submit'
        },
        submitButtonIcon: {
            type: String,
            required: false,
            default: null
        }
    },
    methods: {
        submit() {
            this.$emit('formSubmitted')
        },
        formatFieldTitle(text) {
            return text.charAt(0).toUpperCase() + text.slice(1)
        }
    }
}
</script>

<style scoped>
@import '@/assets/colors.css';
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
    font-weight: 700;
}
.playButton {
    display: block;
    margin-top: 3ch;
    margin-left: auto;
    margin-right: 0;
}
</style>
