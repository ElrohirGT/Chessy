<template>
    <main>
        <section class="create-room">
            <div>
                <button-important style="">Hello There</button-important>
                <button-hero style="">Hello There</button-hero>
            </div>
        </section>
        <section class="join-room">
            <div></div>
        </section>
    </main>
</template>

<script>
import buttonImportant from '@/components/molecules/buttonImportant.vue'
import buttonHero from '@/components/molecules/buttonHero.vue'

export default {
    components: {
        buttonImportant,
        buttonHero
    }
}
</script>

<style scoped>
main {
    width: 100vw;
    height: 100vh;
    position: absolute;
    display: flex;
    flex-direction: row;
}

.create-room {
    background-color: rgba(205, 158, 158, 0.568);
    width: 50%;
}
.join-room {
    background-color: rgba(0, 0, 255, 0.364);
    width: 50%;
}
</style>
