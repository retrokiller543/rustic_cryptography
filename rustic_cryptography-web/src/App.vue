<script lang="ts">
import { RouterView } from 'vue-router'
import Navbar from './components/Navbar.vue'
import AppLoader from './AppLoader.vue'
import { defineComponent } from 'vue';

export default defineComponent({
  components: {
    Navbar,
    RouterView,
    AppLoader
  },

  data() {
    return {
      loading: true
    }
  },

  mounted() {
    this.loading = false
  }
})
</script>

<template>
  <AppLoader v-if="loading" />

  <div v-else class="main-container">
    <header>
      <Navbar />
    </header>

    <main class="content">
      <RouterView />
    </main>

    <footer class="footer">
      <div class="content has-text-centered">
        <p>
          <strong>Shielded Rust</strong> by <a href="https://jgthms.com">Emil Schütt</a>.
        </p>
      </div>
    </footer>
  </div>
</template>

<style scoped>
html,
body {
  height: 100%;
  margin: 0;
}

.main-container {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
}

.content {
  flex-grow: 1;
}

.footer {
  z-index: 0;
}
</style>