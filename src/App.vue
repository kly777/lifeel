<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface Todo {
  id: number;
  title: string;
  completed: boolean;
}

const todos = ref<Todo[]>([]);

async function fetchTodos() {
  try {
    const result = await invoke("fetch_todos") as Todo[];
    console.log("Fetched todos:", result);
    todos.value = result
  } catch (error) {
    console.error("Error fetching todos:", error);
  }
}
// 新增响应式数据
const newTodoTitle = ref("");

// 新增创建函数
async function addTodo() {
  if (!newTodoTitle.value.trim()) return;

  try {
    await invoke("create_todo", {
      title: newTodoTitle.value.trim(),
      completed: false // 默认未完成
    });
    newTodoTitle.value = "";
    await fetchTodos(); // 刷新列表
  } catch (error) {
    console.error("创建失败:", error);
  }
}

async function deleteTodo(id: number) {
  try {
    await invoke("delete_todo", { id });
    await fetchTodos(); // 刷新列表
  } catch (error) {
    console.error("删除失败:", error);
  }
}
fetchTodos();
</script>

<template>
  <main class="container">
    <h1>Welcome</h1>
    <form class="row" @submit.prevent="addTodo">
      <input v-model="newTodoTitle" placeholder="输入待办事项..." />
      <button type="submit">添加</button>
    </form>


    <ul>
      <h1>Todo</h1>
      <li v-for="todo in todos" :key="todo.id">
        {{ todo.title }} - {{ todo.completed ? "已完成" : "未完成" }}
        <button @click="deleteTodo(todo.id)" class="delete-btn">×</button>
      </li>
    </ul>
  </main>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

.delete-btn {
  margin-left: 10px;
  color: #ff4444;
  background: none;
  border: none;
  cursor: pointer;
}
</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}

button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }

  button:active {
    background-color: #0f0f0f69;
  }
}
</style>