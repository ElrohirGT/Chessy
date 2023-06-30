<template>
    <input @input="updateValue" :type="this.inputType" :placeholder="this.placeholder" />
</template>

<script>
/**
 * Representation of a form's field. Code to works with v-model
 */
export default {
    props: {
        /**
         * Required for v-model behaviour.
         */
        modelValue: {},
        /**
         * Set the input tag type.
         * @values text, password, range...
         */
        inputType: {
            required: true,
            type: String
        },
        /**
         * Renders the placeholder value for some input types.
         */
        placeholder: {
            required: false,
            type: String,
            default: 'Insert a value'
        }
    },

    emits: ['update:modelValue'],

    data() {
        return {
            currentValue: null
        }
    },
    methods: {
        /**
         * Call when the input's value is modify.
         * @param {event} event Action of changing INPUT's value.
         */
        updateValue(event) {
            // Update value
            this.currentValue = event.target.value
            // Emit the value to parent
            this.$emit('update:modelValue', this.currentValue)
        }
    }
}
</script>

<style scoped>
input {
    display: inline-block;
    height: 3.5rem;
    padding: 0 3ch;
    border: 2px solid #ccc;
    border-left: 0;
    width: 40ch;
    color: #666;
    border-radius: 7px;
    font-family: 'PT Sans', sans-serif;
    font-size: 1rem;
}
input::placeholder {
    font-style: italic;
}
</style>
