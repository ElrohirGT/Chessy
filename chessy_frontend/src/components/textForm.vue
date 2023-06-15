<template>
  <div class="formBox">
    <h3>{{ title }}</h3>
    <form v-for="(field, key) in fields" :key="key">
      <input v-model="field.currentValue" type="text" :placeholder="field.fieldName" />
    </form>
    <button @click="submitForm">{{ submitButtonContent }}</button>
  </div>
</template>

<script>
//import formField from './formField'
import { isProxy, toRaw } from 'vue'

export default {
  props: {
    fields: {
      type: Array,
      required: true
    },
    title: {
      type: String,
      required: true
    },
    submitButtonContent: {
      type: String,
      required: false,
      default() {
        return 'Submit'
      }
    }
  },
  methods: {
    submitForm() {
      this.$emit('submitForm')
    }
  },
  computed: {
    missingFields() {
      return ''
    }
  }
}
</script>

<style scoped>
.formBox {
  padding: 2ch;
  display: block;
  width: max-content;
  background-color: white;
  box-shadow: rgba(0, 0, 0, 0.16) 0px 10px 36px 0px, rgba(0, 0, 0, 0.06) 0px 0px 0px 1px;
}
</style>
