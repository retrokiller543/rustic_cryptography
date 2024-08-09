import { ref, computed } from 'vue'
import { defineStore } from 'pinia'
import { Config, get_default_config } from 'shielded_rust'

export const useConfigStore = defineStore('config', () => {
  const config = ref(get_default_config());
  const alfabet = computed(() => config.value.alfabet);
  const base64 = computed(() => config.value.base64);

  function setConfig(newConfig: Config) {
    config.value = newConfig
  }

  return { config, alfabet, base64, setConfig }
})
