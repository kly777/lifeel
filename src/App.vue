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
      <input v-model.trim="newTodoTitle" placeholder="输入待办事项..." />
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
:root {
  font-family: 'Poppins', Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #f8f8f8;
  background-color: #f9f9f9;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0 auto;
  padding-top: 10vh;
  max-width: 600px;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  text-align: center;
  background: #ffffff;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  border-radius: 12px;
  padding: 2rem;
}

h1 {
  font-size: 2rem;
  color: #396cd8;
  margin-bottom: 1rem;
}

.row {
  display: flex;
  justify-content: center;
  gap: 0.5rem;
  margin-bottom: 1.5rem;
}

input {
  flex: 1;
  border-radius: 8px;
  border: 1px solid #ddd;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-family: inherit;
  color: #333;
  background-color: #f9f9f9;
  transition: border-color 0.25s, box-shadow 0.25s;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

input:focus {
  border-color: #396cd8;
  box-shadow: 0 0 4px rgba(57, 108, 216, 0.5);
}

button {
  border-radius: 8px;
  border: none;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 600;
  font-family: inherit;
  color: #fff;
  background-color: #396cd8;
  transition: background-color 0.25s, transform 0.1s;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
  cursor: pointer;
}

button:hover {
  background-color: #2f5bbd;
}

button:active {
  transform: scale(0.98);
}

ul {
  list-style: none;
  padding: 0;
  margin: 0;
  width: 100%;
}

li {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: #ffffff;
  border: 1px solid #ddd;
  border-radius: 8px;
  padding: 0.8rem 1rem;
  margin-bottom: 0.5rem;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  transition: transform 0.2s, box-shadow 0.2s;
}

li:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.15);
}

.delete-btn {
  margin-left: 10px;
  color: #ff4444;
  background: none;
  border: none;
  cursor: pointer;
  font-size: 1.2rem;
  transition: transform 0.2s;
}

.delete-btn:hover {
  transform: scale(1.2);
}

/* @media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #1b1b1b;
  }

  .container {
    background: #fbfbfb;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.3);
  }

  input,
  button {
    color: #ffffff;
    background-color: #444;
  }

  li {
    background: #444;
    border-color: #555;
  }

  button:hover {
    background-color: #2f5bbd;
  }
} */
</style>