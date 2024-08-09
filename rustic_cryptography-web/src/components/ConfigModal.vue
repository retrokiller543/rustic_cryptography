<template>
  <div class="modal is-active">
    <Modal @close="close">
      <template #body>
        <div class="box">
          <h2 class="title is-4">Configure Crypto Engine</h2>

          <form @submit.prevent="submitForm">
            <div v-for="(value, key) in formObj" :key="key" class="field">
              <label class="label">{{ key.charAt(0).toUpperCase() + key.slice(1) }}</label>
              <div class="control">
                <input
                  v-if="getInputType(value) !== 'checkbox'"
                  class="input"
                  v-model="form[key]"
                  :type="getInputType(value)"
                  :required="isRequired(value)"
                />
                <label v-else class="checkbox">
                  <input
                    v-model="form[key]"
                    :type="getInputType(value)"
                    :required="isRequired(value)"
                    @change="key === 'base64' && toggleBase64()"
                  />
                  Enable {{ key.charAt(0).toUpperCase() + key.slice(1) }}
                </label>
              </div>
            </div>

            

            <div class="field is-grouped">
              <div class="control">
                <button class="button is-link" type="submit">Save</button>
              </div>
              <div class="control">
                <button class="button is-light" @click="close">Cancel</button>
              </div>
            </div>
          </form>
        </div>
      </template>
    </Modal>
  </div>
</template>

<script lang="ts">
import { defineComponent, reactive } from 'vue';
import { Config, get_default_alfabet, get_base64_alfabet } from 'shielded_rust'
import { useConfigStore } from '../stores/config';
import Modal from './common/Modal.vue'

interface ConfigForm {
  alfabet: string;
  base64: boolean;
}

export default defineComponent({
  components: {
    Modal
  },
  emits: ['close', 'configSaved'],
  methods: {
    getComponentType(value: any) {
      if (typeof value === 'boolean') return 'checkbox';
      if (typeof value === 'number') return 'number';
      if (typeof value === 'string') return 'input';

      return 'input';
    },
    getInputType(value: any) {
      if (typeof value === 'boolean') return 'checkbox';
      if (typeof value === 'number') return 'number';
      if (typeof value === 'string') return 'text';

      return 'text';
    },
    isRequired(value: any) {
      if (value === 'alfabet') return true;
      return false;
    },
    toggleBase64() {
      if (this.form.base64) {
        this.form.alfabet = get_base64_alfabet();
      } else {
        this.form.alfabet = get_default_alfabet();
      }
    },
    toggleUTF8() {
      if (this.form.utf8) {
        this.form = this.form.with_utf8();
      } else {
        this.form.alfabet = get_default_alfabet();
      }
    }
  },
  computed: {
    formObj() {
      return {
        alfabet: this.form.alfabet,
        base64: this.form.base64
      }
    }
  },
  setup(props, { emit }) {
    const configStore = useConfigStore();
    let form: Config = reactive(configStore.config);
    const formType: ConfigForm = reactive({
      alfabet: form.alfabet,
      base64: form.base64
    })

    const close = () => {
      emit('close');
    };

    const submitForm = () => {
      emit('configSaved', form);
      configStore.setConfig(form);
      close();
    };

    return {
      form,
      formType,
      close,
      submitForm,
    };
  },
});
</script>