<template>
    <input @input="updateValue" :type="this.inputType" :placeholder="this.placeholder" />
</template>

<script>
export default {
    props: {
        modelValue: {},
        inputType: {
            required: true,
            type: String
        },
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
