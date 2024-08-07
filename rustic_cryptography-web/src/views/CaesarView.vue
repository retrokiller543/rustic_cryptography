<template>
    <main>
        <CryptoView @configSaved="configSaved">
            <h1 class="title">Caesar</h1>
            <article class="section">
                <section class="section">
                    <h2 class="title">Encrypt</h2>
                    <form @submit.prevent="submitForm">
                        <div class="field">
                            <label class="label">Key</label>
                            <div class="control is-flex">
                                <input class="input" type="number" v-model="form.key" />
                                <button class="button is-info ml-2" type="button" @click="generateRandomKey">
                                    Generate Random Key
                                </button>
                            </div>
                        </div>

                        <div class="field">
                            <label class="label">Plaintext</label>
                            <div class="control">
                                <input class="input" type="text" v-model="form.plaintext" />
                            </div>
                        </div>

                        <div class="field is-grouped">
                            <div class="control">
                                <button class="button is-info" type="submit">Encrypt</button>
                            </div>
                        </div>
                    </form>
                </section>

                <section class="section">
                    <h2 class="title">Results</h2>
                    <p class="subtitle">{{ encrypted_str }}</p>
                    <p class="subtitle">Encryption Time: {{ encryptionTime }} ms</p>
                </section>
            </article>
        </CryptoView>
    </main>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { Config, encrypt_string } from 'rustic_cryptography';
import CryptoView from '../components/CryptoView.vue';
import { useConfigStore } from '@/stores/config';
import timingMixin from '@/mixins/timingMixin';

export default defineComponent({
    components: {
        CryptoView,
    },
    mixins: [timingMixin],
    data() {
        return {
            form: {
                key: 0,
                plaintext: '',
            },
            encrypted: ''
        };
    },
    computed: {
        config() {
            return this.configStore.config;
        },
        encrypted_str(): string {
            return this.encrypted;
        }
    },
    methods: {
        configSaved(config: Config) {
            console.log(config);
        },
        submitForm() {
            this.encryptionTime = this.timeOperation(() => {
                this.encrypted = encrypt_string(this.form.plaintext, this.configStore.config, this.form.key);
            });
        },
        generateRandomKey() {
            this.form.key = Math.floor(Math.random() * this.config.alfabet.length) + 1;
        }
    },
    setup() {
        const configStore = useConfigStore();
        return {
            configStore,
        };
    }
});
</script>

<style lang="scss" scoped>
main {
    height: 100vh;
}
</style>