<template>
  <main>
    <h1>Login View</h1>
    <textForm :fields="this.fields" title="Login" @submit-form="registerUser"></textForm>
  </main>
</template>

<script>
import formField from '../components/formField.js'
import textForm from '../components/textForm.vue'
import {REGISTER_URL} from '../main.js'

export default {
  components: {
    textForm
  },
  data() {
    return {
      fields: [new formField('username')]
    }
  },
  methods: {
    async registerUser() {
        let options = {
              method: "POST",
              headers: { "Content-Type": "text/plain" },
              body: this.fields[0].currentValue,
            };

        fetch( REGISTER_URL, options)
        .then(response=>(response.text()))
        .then(text=>(console.log(text)))
    }
  }
}
</script>

<style scoped></style>
