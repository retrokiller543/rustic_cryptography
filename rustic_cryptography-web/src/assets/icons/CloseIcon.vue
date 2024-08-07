<template>
    <svg
      :xmlns="xmlns"
      :height="height"
      :viewBox="viewBox"
      :width="width"
      :fill="fillColor"
    >
      <path d="m256-200-56-56 224-224-224-224 56-56 224 224 224-224 56 56-224 224 224 224-56 56-224-224-224 224Z" />
    </svg>
  </template>
  
  <script>
  export default {
    name: 'DynamicColorSvg',
    props: {
      fillColor: {
        type: String,
        default: 'var(--bulma-text, #ffff)'
      },
      xmlns: {
        type: String,
        default: 'http://www.w3.org/2000/svg'
      },
      height: {
        type: String,
        default: '24px'
      },
      width: {
        type: String,
        default: '24px'
      },
      viewBox: {
        type: String,
        default: '0 -960 960 960'
      }
    }
  }
  </script>