<template>
    <div class="modal is-active" :class="modalSizeClass" @click.stop>
        <div class="modal-background" @click="close"></div>
        <div class="modal-content" :class="modalContentClass">
            <IconButton @click="close" class="close-button corner is-small" :icon="closeIcon"/>
            <header class="modal-card-head" v-if="$slots.header">
                <slot name="header"></slot>
            </header>
            <section class="modal-card-body" v-if="$slots.body">
                <slot name="body"></slot>
            </section>
            <footer class="modal-card-footer" v-if="$slots.footer">
                <slot name="footer"></slot>
            </footer>
        </div>
        <IconButton @click="close" class="modal-close" :icon="closeIcon"/>
    </div>
</template>

<script>
import CloseIcon from '@/assets/icons/CloseIcon.vue';

export default {
    props: {
        size: {
            type: String,
            default: 'medium',
            validator: value => ['small', 'medium', 'large'].includes(value)
        },
        borderRadius: {
            type: String,
            default: '32px'
        }
    },
    methods: {
        close() {
            this.$emit('close');
        },
        handleKeydown(event) {
            if (event.key === 'Escape') {
                this.close();
            }
        }
    },
    mounted() {
        document.addEventListener('keydown', this.handleKeydown);
    },
    beforeDestroy() {
        document.removeEventListener('keydown', this.handleKeydown);
    },
    computed: {
        closeIcon() {
            return CloseIcon;
        },
        modalSizeClass() {
            return {
                'is-small': this.size === 'small',
                'is-medium': this.size === 'medium',
                'is-large': this.size === 'large'
            };
        },
        modalContentClass() {
            return {
                'has-border-radius': true
            };
        }
    }
};
</script>

<style scoped lang="scss">
.modal-background {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
}

.modal-content {
    padding: 1.5rem;
    border-radius: var(--modal-border-radius, 32px);
    position: relative;
    max-width: 90%;
    max-height: 90%;
    overflow: auto;

    &.has-border-radius {
        border-radius: var(--modal-border-radius, 32px);
    }

    &::-webkit-scrollbar {
        width: 0px;
    }

    &::-webkit-scrollbar-track {
        background: #f1f1f1;
        border-radius: 10px;
    }

    &::-webkit-scrollbar-thumb {
        background: #888;
        border-radius: 10px;

        &:hover {
            background: #555;
        }
    }
}

.close-button {
    color: --text;
    border: none;
    font-size: 3rem;
    cursor: pointer;
    z-index: 1;
    background-color: transparent;

    &.corner {
        position: absolute;
        top: 40px;
        right: 30px;
    }
}

.modal-card-head,
.modal-card-body,
.modal-card-footer {
    margin: 1rem 0;
    border-radius: var(--modal-border-radius, 32px);
}
</style>