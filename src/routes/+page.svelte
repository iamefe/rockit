<script lang="ts">
  import { onMount } from "svelte";
  import type { Task } from "../lib/types";
  import { invoke } from "@tauri-apps/api/core";
  import "../app.css";
  import { error } from "@sveltejs/kit";

  let tasks: Task[] = [];
  let oneTask: Task | null = null;
  let taskId: number;
  let description: string = "";
  let id: number;
  let completed: boolean = false;
  let new_description: string;

  const fetchTasks = async () => {
    try {
      tasks = await invoke("fetch_tasks");
      console.log("Tasks fetched", tasks);
    } catch (error) {
      console.error("Error fetching tasks", error);
    }
  };

  const addTask = async () => {
    try {
      if (!description) {
        console.error("Description cannot be empty: ", error);
        return;
      }
      await invoke("create_task", { description });
      description = "";
      fetchTasks();
    } catch (error) {
      console.error("Error adding task", error);
    }
  };

  const updateTask = async () => {
    if (!new_description) {
      console.error("Description cannot be empty: ", error);
      return;
    }
    try {
      await invoke("edit_task", {
        task_id: id,
        new_description,
      });
      new_description = "";
    } catch (error) {
      console.error("Error updating task", error);
    }
    id = NaN;
    completed = false;
    fetchTasks();
  };

  const getTaskById = async () => {
    console.log("Fetching task by id:", typeof taskId);
    const parsedId = parseInt(String(taskId));
    console.log("ParsedId type:", typeof parsedId);
    if (isNaN(parsedId) || parsedId <= 0) {
      console.error("Invalid task ID");
      return;
    }

    try {
      console.log("Trying to invoke");
      oneTask = await invoke<Task | null>("fetch_task_by_id", {
        task_id: parsedId,
      });
      console.log("OneTask's returned value: ", oneTask);
      if (!oneTask) {
        console.log("No task found with ID:", parsedId);
      } else {
        console.log("Fetched task by id:", oneTask?.description);
      }
    } catch (error) {
      console.error("Error fetching task by id:", error);
    }
  };

  const deleteTask = async (id: number) => {
    try {
      await invoke("remove_task", { task_id: id });
      fetchTasks();
    } catch (error) {
      console.error("Error deleting task", error);
    }
  };

  const toggleComplete = async () => {
    completed = !completed;
    try {
      await invoke("edit_task", {
        task_id: id,
        completed,
      });

      fetchTasks();
    } catch (error) {
      console.error("Error updating task", error);
    }
    id = NaN;
    completed = false;
    new_description = "";
  };

  onMount(fetchTasks);
</script>

<main class="flex flex-col items-center">
  <div class="pt-28 w-[700px]">
    <header class="flex flex-row mb-16 gap-3 justify-between items-end">
      <h1 class="font-bold text-5xl">🚀️ RockIt</h1>
      <p class="text-gray-500">Developed by Oserefemhen Ativie</p>
    </header>

    <div class="gap-10 flex flex-col mb-10">
      <div class="grid grid-cols-1 gap-4">
        <p class="font-medium">What do you want to do today?</p>
        <form on:submit|preventDefault={addTask} class="flex gap-4 flex-col">
          <input
            bind:value={description}
            placeholder="Road trip round the world"
            class="border-gray-200 border-2 rounded-full px-5 py-2 w-full block"
          />
          <button
            type="submit"
            class="bg-green-800 hover:bg-green-700 w-fit text-white px-5 py-[10px] block rounded-full font-medium transition-all"
            >Add task</button
          >
        </form>
      </div>

      {#if tasks.length > 0}
        <div class="grid grid-cols-1 gap-4">
          <p class="font-medium">Edit a task</p>
          <form
            on:submit|preventDefault={updateTask}
            class="flex flex-col gap-4 mb-10"
          >
            <div class="flex flex-row items-center gap-2">
              <!-- <input
              bind:value={id}
              type="number"
              placeholder="Task id"
              class="border-gray-200 border-2 rounded-full pl-5 pr-2 py-2 w-32"
            /> -->
              <input
                bind:value={new_description}
                placeholder="New task description"
                class="border-gray-200 border-2 w-full rounded-full px-5 py-2"
              />
              <!-- <input
              bind:checked={completed}
              type="checkbox"
              class="w-12 h-[44px] rounded-2xl"
            /> -->
            </div>

            <button
              type="submit"
              class="bg-green-800 w-fit hover:bg-green-700 text-white px-5 py-[10px] rounded-full font-medium transition-all"
              >Update task</button
            >
          </form>
        </div>
      {/if}
    </div>

    <!-- <section class="mb-10">
      <h3 class="font-bold text-xl mb-4">Search for a task</h3>
      <form
        on:submit|preventDefault={getTaskById}
        class="flex flex-row items-center gap-2 mb-5"
      >
        <input
          bind:value={taskId}
          type="number"
          placeholder="Task ID"
          class="border-gray-200 border-2 rounded-full pl-5 pr-2 py-2 w-32"
        />
        <button
          type="submit"
          class="bg-green-800 hover:bg-green-700 text-white px-5 py-[10px] rounded-full font-bold transition-all"
          >Find task</button
        >
      </form>

      {#if oneTask}
        <p>
          {oneTask.description}
        </p>
      {:else}
        <p>No task found.</p>
      {/if}
    </section> -->

    <section>
      <h3 class="font-medium text-xl mb-4">All tasks</h3>
      {#if tasks.length > 0}
        {#each tasks as task}
          <div class="flex flex-row items-center gap-3 mb-3">
            <a
              href={`#`}
              on:click={() => {
                id = task.id;
                completed = task.completed;
                toggleComplete();
              }}
            >
              <span class="text-gray-500">╰┈➤</span>
              <span class="font-bold inline">
                {`🚀️ `}
              </span>
              &nbsp;
              <span class={task.completed ? `line-through text-gray-600` : ""}
                >{`${task.description}`}</span
              >
            </a>
            <div class="inline">
              <button
                on:click={() => {
                  new_description = task.description;
                  id = task.id;
                }}
                class="border-2 text-gray-500 hover:text-gray-700 px-5 py-1 rounded-full font-medium transition-all"
                >Edit</button
              >
              <button
                on:click={() => deleteTask(task.id)}
                class="border-2 text-gray-500 hover:text-gray-700 px-5 py-1 rounded-full font-medium transition-all"
                >Delete</button
              >
            </div>
          </div>
        {/each}
      {:else}
        <p>🚀️ Add a new task.</p>
      {/if}
    </section>
  </div>
</main>
