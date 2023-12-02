<script setup>
import { RouterView } from 'vue-router'
</script>

<template>
    <RouterView />
    <button class="btn">Play</button>
    <div class="shape red-fill jelly">
        <svg x="0px" y="0px" viewBox="0 0 108.1 47" enable-background="new 0 0 108.1 47">
            <polygon fill="#FF0000" points="0,7.1 127.3,0 32.3,64 4.8,58.2"/>
        </svg></div>  
      <div class="shape red-fill jelly"><svg x="0px" y="0px"
     viewBox="0 0 108.1 47" enable-background="new 0 0 108.1 47">
<polygon fill="#FF0000" points="15.5,0 70.7,0 118.1,32.7 43.1,47 "/>
</svg></div>
</template>

<style scoped>

.btn{
        
}

.btn:hover{
    cursor: pointer;
    animation: tiltBounce 0.15s ease;
    animation-fill-mode: forwards;
}

@keyframes tiltBounce {
  0% {
    transform: scale(1) rotate(-7deg);
  }
  50% {
    transform: scale(1.5) rotate(-7deg);
  }
  100% {
    transform: scale(1.15) rotate(-7deg);
  }
}

@keyframes tiltBounceAlt {
  0% {
    transform: scale(1) rotate(7deg);
  }
  50% {
    transform: scale(1.5) rotate(7deg);
  }
  100% {
    transform: scale(1.15) rotate(7deg);
  }
}
</style>
