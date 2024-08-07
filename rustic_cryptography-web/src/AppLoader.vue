<template>
  <div>
    <div className="section">
      <div className="columns">
        <div className="column is-6 is-offset-3">
          <div className="loader-wrapper is-active">
            <div className="loader is-loading" />
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
  background: #fff;
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
  border: 4px solid var(--color-skolon);
  border-right-color: transparent;
  border-top-color: transparent;
}
</style>