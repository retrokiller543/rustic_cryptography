<template>
  <div>
    <div class="section">
      <div class="columns">
        <div class="column is-6 is-offset-3">
          <div class="loader-wrapper is-active">
            <div class="loader is-loading" />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {

}
</script>

<style scoped>
.loader-wrapper {
  position: absolute;
  top: 0;
  left: 0;
  height: 100%;
  width: 100%;
  background: var(--bulma-background);
  opacity: 0;
  z-index: -1;
  transition: opacity 0.3s;
  display: flex;
  justify-content: center;
  align-items: center;
  border-radius: 6px;
}

.loader-wrapper.is-active {
  opacity: 1;
  z-index: 1;
}

.loader-wrapper .loader {
  height: 80px;
  width: 80px;
  border: 4px solid var(--bulma-primary);
  border-right-color: transparent;
  border-top-color: transparent;
}
</style>