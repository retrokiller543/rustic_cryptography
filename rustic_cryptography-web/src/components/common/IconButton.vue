<template>
    <button class="button custom-button" :title="title">
      <component :is="icon" />
    </button>
  </template>
  
  <script>
  export default {
    props: {
      title: {
        type: String,
        default: ''
      },
      icon: {
        default: null,
        required: true
      },
      iconColor: {
        type: String,
        default: '#000000'
      }
    }
  }
  </script>
  
  <style>
  .custom-button {
    height: 40px;
    width: 40px;
    padding: 0;
    border-radius: 50%;
    box-shadow: #00000029 0 3px 6px;
    transform: scale(1);
    transition: all 0.2s ease-in-out;
    box-shadow: none;
    border-width: 0px;
  }
  </style>