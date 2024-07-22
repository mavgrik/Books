<script setup>
import { invoke } from "@tauri-apps/api/core";

function sendPanic() {
    invoke("panic");
}
</script>

<template>
  <form class="row" @submit.prevent="sendPanic">
    <button type="submit">Panic</button>
  </form>
</template>
