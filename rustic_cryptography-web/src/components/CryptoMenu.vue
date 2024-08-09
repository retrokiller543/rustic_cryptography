<template>
    <div class="crypto-menu">
        <aside :class="['menu', { 'is-open': isMenuOpen }]">
            <button @click="toggleMenu" class="menu-toggle-button">
                <!-- You can use any icon or text here -->
                <span v-if="isMenuOpen">Close</span>
                <span v-else>Open</span>
            </button>
            <p class="menu-label">Symmetric Ciphers</p>
            <ul class="menu-list">
                <li><router-link to="/caesar">Caesar Cipher</router-link></li>
                <li><router-link to="/vigenere">Vigenere Cipher</router-link></li>
            </ul>
            <p class="menu-label">Asymmetric Ciphers</p>
            <ul class="menu-list">
                <li><router-link to="/rsa">RSA</router-link></li>
            </ul>
        </aside>
    </div>
</template>

<script>
export default {
    name: 'CryptoMenu',
    data() {
        return {
            isMenuOpen: false,
        };
    },
    methods: {
        toggleMenu() {
            this.isMenuOpen = !this.isMenuOpen;
        },
    },
};
</script>

<style scoped lang="scss">

li {
    list-style: none;
}

.crypto-menu {
    display: flex;

    .menu {
        width: 250px;
        transition: transform 0.3s ease-in-out;
        transform: translateX(-100%);
        position: relative;
        
        @media (max-width: 768px) {
            width: 100%;
            transform: translateX(-100%);
        }



        &.is-open {
            transform: translateX(0);
        }

        .menu-toggle-button {
            position: absolute;
            top: 0;
            right: -50px;
            width: 50px;
            height: 100%;
            background-color: --primary;
            color: --white;
            border: none;
            cursor: pointer;
            display: flex;
            align-items: center;
            justify-content: center;
            writing-mode: vertical-rl;
            text-orientation: mixed;

            &:focus {
                outline: none;
            }

            &:hover {
                background-color: --primary-color-dark;
            }

            span {
                font-weight: bold;
            }
        }

        .menu-label {
            padding: 1em;
            background-color: --primary-color-dark;
        }

        .menu-list {
            padding-left: 1em;

            li {
                margin-bottom: 0.5em;

                a {
                    color: --white;
                    text-decoration: none;

                    &:hover {
                        text-decoration: underline;
                    }
                }
            }
        }
    }
}
</style>