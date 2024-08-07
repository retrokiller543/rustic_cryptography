<template>
    <main class="is-flex">
      <CryptoMenu />
      <section class="section">
        <slot></slot>
      </section>
      <button
        @click="showModal = true"
        class="button is-primary is-fixed-bottom-right"
      >
        Config
      </button>
      <ConfigModal v-if="showModal" @close="closeModal" @submit="submitConfig" />
    </main>
  </template>
  
  <script lang="ts">
  import { ref } from 'vue';
  import CryptoMenu from './CryptoMenu.vue';
  import ConfigModal from './ConfigModal.vue';
  import { Config } from 'rustic_cryptography';
  
  export default {
    components: {
      CryptoMenu,
      ConfigModal,
    },

    props: {
      onConfigSaved: {
        type: Function,
      },
    },

    methods: {
      submitConfig(config: Config) {
        this.$emit('configSaved', config);
        this.closeModal();
      }
    },
    emits: ['configSaved'],
    setup() {
      const showModal = ref(false);
  
      const closeModal = () => {
        showModal.value = false;
      };
  
      return {
        showModal,
        closeModal,
      };
    },
  };
  </script>
  
  <style scoped>
  .is-fixed-bottom-right {
    position: fixed;
    bottom: 1rem;
    right: 1rem;
  }
  </style>