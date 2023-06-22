<template>
  <button class="pushable">
    <span class="shadow"></span>
    <span class="edge"></span>
    <span class="front">
      <slot></slot>
    </span>
  </button>
</template>

<script>
export default {
  setup() {
    return {}
  }
}
</script>

<style scoped>
@import '../../assets/colors';

.front {
  display: block;
  padding: 1ch 2ch;
  border-radius: 12px;
  background-color: var(--primary-button);
  color: var(--text);
  transform: translateY(-4px);
  font-size: 1rem;
  font-weight: bold;
  will-change: transform;
  transition: transform 250ms;
}
.pushable {
  position: relative;
  background: transparent;
  border: none;
  padding: 0;
  cursor: pointer;
  outline-offset: 5px;
  -webkit-tap-highlight-color: transparent;
}

.shadow {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  border-radius: 12px;
  background-color: hsl(0deg 0% 0% / 0.25);
  transform: translateY(2px);
}
.edge {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  border-radius: 12px;
  background: var(--primary-button-dark);
}

.pushable:focus:not(:focus-visible) {
  outline: none;
}
.pushable:hover .front {
  transform: translateY(-6px);
  transition: transform 250ms cubic-bezier(0.3, 0.7, 0.4, 1.5);
}
.pushable:active .front {
  transform: translateY(-2px);
  transition: transform 34ms;
}
.pushable:hover .shadow {
  transform: translateY(4px);
  transition: transform 250ms cubic-bezier(0.3, 0.7, 0.4, 1.5);
}
.pushable:active .shadow {
  transform: translateY(1px);
  transition: transform 34ms;
}
</style>
