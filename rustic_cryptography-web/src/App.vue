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

  <div v-else>
    <header>
    <Navbar />
  </header>

  <RouterView />

  <footer class="footer">

  </footer>
  </div>

</template>

<style scoped>
main {
  height: max-content;
}
</style>
