<script setup>
import { ref } from 'vue';
import { my_project_backend } from 'declarations/my_project_backend/index';
let blogs = ref([]);
let tags = ref([])

async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const title = target.querySelector('#title').value;
  const content = target.querySelector('#content').value;

  await my_project_backend.add_blog(title, content, tags.value);
  await getBlogs()
  tags.value = []
}

async function getBlogs() {
  blogs.value = (await my_project_backend.get_blogs()).map(blog => {
    const date = new Date(Number(blog.date / 1_000_000n))
    return {
      ...blog,
      date,
    }
  }); 
}

function addTag(target) {
  if(target.key === "Enter"){
    target.stopPropagation()
    target.preventDefault()
    const found = tags.value.findIndex(tag => tag === target.target.value)
    if(found === -1){
      tags.value.push(target.target.value)
    }
    else{
      tags.value.splice(found, 1)
    }
    target.target.value = ""
  }
}

</script>

<template>
  <main class="container mx-auto">
    <img src="/logo2.svg" alt="DFINITY logo" class="mx-auto mt-4" />
    <br />
    <br />
    <form id="main-form" action="#" @submit="handleSubmit" class="flex flex-col items-stretch">
      <span>
        <label for="title">Title:</label> <br/>
        <input id="title" alt="Name" type="text" />
      </span>

      <span>
        <label for="content">Content:</label> <br/>
        <textarea id="content" alt="content" type="text"></textarea>
      </span>

      <span>
        <label for="tags">tags:</label> <br/>
        <input id="tags" alt="tags" type="text" v-on:keypress="addTag" />
      </span>

      <span class="text-white flex gap-5 mt-5 font-bold">
        <p class="rounded-full px-2 py-2 bg-purple-600" v-for="tag of tags">{{ tag }}</p>
      </span>

      <button type="submit" class="bg-emerald-800 py-4 px-2 rounded-lg mt-1">Click to add!</button>
    </form>

    <div class="mt-10 flex flex-col gap-10">
      <section v-for="blog of blogs">
        <header class="flex flex-col text-white font-bold bg-emerald-600 py-3 px-3">
          <span class="flex text-4xl">
            <h1 class="flex-grow">{{ blog.title }}</h1>
            <h1>{{ blog.date.toISOString().split('T')[0] }}</h1>
          </span>
          <span class="flex flex-row gap-1">
            <div v-for="tag of blog.tags" class="rounded-full bg-purple-600 px-2">{{ tag }}</div>
          </span>
        </header>
        <p class="bg-white py-2 px-2 min-h-[15rem]">{{ blog.content }}</p>
      </section>
    </div>
  </main>
</template>
