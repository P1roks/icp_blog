<script setup>
import { ref } from 'vue';
import { my_project_backend } from 'declarations/my_project_backend/index';
let blogs = ref([]);

async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const title = target.querySelector('#title').value;
  const content = target.querySelector('#content').value;
  const tags = target.querySelector('#tags').value.split(",");

  await my_project_backend.add_blog(title, content, tags);
  await getBlogs()
}

async function getBlogs() {
  blogs.value = (await my_project_backend.get_blogs()).map(blog => {
    return {
      ...blog,
      date: blog.date.toString()
    }
  }); 
}

</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    <form action="#" @submit="handleSubmit">
      <span>
        <label for="title">Title:</label>
        <input id="title" alt="Name" type="text" />
      </span>

      <span>
        <label for="content">Content:</label>
        <input id="content" alt="content" type="text" />
      </span>

      <span>
        <label for="tags">tags:</label>
        <input id="tags" alt="tags" type="text" />
      </span>

      <button type="submit">Click to add!</button>
    </form>

    <div>
      <section v-for="blog of blogs">
        <h1>{{ blog.title }}</h1>
        <p>{{ blog.content }}</p>
      </section>
    </div>
  </main>
</template>
