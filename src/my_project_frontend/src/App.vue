<script setup>
import { ref } from 'vue';
import { my_project_backend } from 'declarations/my_project_backend/index';
let chat = ref([]);

async function getChats(){
  chat.value = await my_project_backend.get_chat();
}


async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const chat = target.querySelector('#chat').value;
  await my_project_backend.save_chat(chat);
  await getChats()
}

getChats()
</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    <form action="#" @submit="handleSubmit">
      <label for="chat">Enter your message: &nbsp;</label>
      <input id="chat" alt="chat" type="text" />
      <button type="submit">Click Me!</button>
    </form>
    <section id="chat">{{ chat }}</section>
  </main>
</template>