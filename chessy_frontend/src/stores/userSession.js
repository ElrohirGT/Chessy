import { ref, computed } from 'vue'
import { defineStore } from 'pinia'

export const useUserSessionStore = defineStore('userSession', () => {
    const userCredentials = ref({
        name: null,
        id: null
    })
    const isUserValidated = computed(() => {
        if (userCredentials.value.id == null) return false
        else if (userCredentials.value.name == null) return false
        return true
    })

    function setUserCredentials({ name, id }) {
        userCredentials.value.name = name
        userCredentials.value.id = id
    }

    return { userCredentials, isUserValidated, setUserCredentials }
})
