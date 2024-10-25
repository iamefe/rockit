<script lang="ts">
  import { onMount } from "svelte";
  import type { Task } from "../lib/types";
  import { invoke } from "@tauri-apps/api/core";
  import "../app.css";
  import { error } from "@sveltejs/kit";

  let tasks: Task[] = [];
  // let oneTask: Task | null = null;
  // let taskId: number;
  let description: string = "";
  let id: number;
  let completed: boolean = false;
  let new_description: string;
  let updating: boolean = false;

  const sortTasksById = () => {
    tasks.sort((a, b) => b.id - a.id);
  };

  const fetchTasks = async () => {
    try {
      tasks = await invoke("fetch_tasks");
      tasks.map((task) => (task.deleting = false));
      sortTasksById();
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
      updating = false;
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
    updating = false;
    fetchTasks();
  };

  // const getTaskById = async () => {
  //   console.log("Fetching task by id:", typeof taskId);
  //   const parsedId = parseInt(String(taskId));
  //   console.log("ParsedId type:", typeof parsedId);
  //   if (isNaN(parsedId) || parsedId <= 0) {
  //     console.error("Invalid task ID");
  //     return;
  //   }

  //   try {
  //     console.log("Trying to invoke");
  //     oneTask = await invoke<Task | null>("fetch_task_by_id", {
  //       task_id: parsedId,
  //     });
  //     console.log("OneTask's returned value: ", oneTask);
  //     if (!oneTask) {
  //       console.log("No task found with ID:", parsedId);
  //     } else {
  //       console.log("Fetched task by id:", oneTask?.description);
  //     }
  //   } catch (error) {
  //     console.error("Error fetching task by id:", error);
  //   }
  // };

  const deleteTask = async (id: number) => {
    try {
      await invoke("remove_task", { task_id: id });
      fetchTasks();
    } catch (error) {
      console.error("Error deleting task", error);
    }
    // deleting = false;
  };

  const toggleDeleting = (task_param: Task) => {
    // For future reference: This works (long form)
    // tasks = tasks.map((task) => {
    //   if (task_param.id === task.id) {
    //     task = { ...task, deleting: !task.deleting };
    //   }
    //   return task;
    // });

    // This works (short form)
    tasks = tasks.map((task) =>
      task_param.id === task.id ? { ...task, deleting: !task.deleting } : task
    );
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
  <div class="pt-20 w-[700px] flex flex-col gap-10">
    <header class="flex flex-row gap-3 justify-between items-end mb-4">
      <h1 class="font-bold text-5xl">🚀️ Rockit</h1>
      <p class="text-gray-500">Developed by Oserefemhen Ativie</p>
    </header>

    <!-- <section class="border-2 rounded-2xl py-4">
      <div class="pl-4 border-b-2 mb-4">
        <h3 class="font-medium text-xl mb-4">Quote</h3>
      </div>
      <div class="flex flex-row gap-2 pl-4">
        <p>
          “Success is the sum of small efforts, repeated day in and day out.”
        </p>
        <p class="text-gray-500">Robert Collier</p>
      </div>
    </section> -->

    <section class="border-2 rounded-2xl py-4">
      <div class="flex flex-row gap-2 pl-5">
        <p>“Discipline is the bridge between goals and accomplishment.”</p>
        <p class="text-gray-500">- Jim Rohn</p>
      </div>
    </section>

    <section class="gap-10 flex flex-col">
      <div class="grid grid-cols-1 gap-4">
        <p class="font-medium">What task do you have in mind?</p>
        <form on:submit|preventDefault={addTask} class="flex gap-4 flex-col">
          <input
            bind:value={description}
            placeholder="Road trip round the world"
            class="border-gray-200 border-2 rounded-full px-5 py-2 w-full block"
            on:click={() => {
              updating = false;
            }}
          />
          <button
            type="submit"
            class="bg-gray-800 hover:bg-gray-700 w-fit text-white px-5 py-[10px] block rounded-full font-medium transition-all"
            >Add task</button
          >
        </form>
      </div>

      {#if tasks.length > 0 && updating}
        <div class="grid grid-cols-1 gap-4">
          <p class="font-medium">Edit task</p>
          <form
            on:submit|preventDefault={updateTask}
            class="flex flex-col gap-4"
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

            <div class="flex flex-row gap-2">
              <button
                type="submit"
                class="bg-gray-800 w-fit hover:bg-gray-700 text-white px-5 py-[10px] rounded-full font-medium transition-all"
                >Update</button
              >

              <button
                on:click={() => {
                  updating = false;
                }}
                class="w-fit text-gray-700 hover:border-gray-200 border-transparent border-2 px-5 py-[10px] rounded-full font-medium transition-all"
                >Cancel</button
              >
            </div>
          </form>
        </div>
      {/if}
    </section>

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

    <section class="border-2 rounded-2xl py-4 mb-16">
      <div class="pl-5 border-b-2 mb-4">
        <h3 class="font-medium text-xl mb-4">Tasks 💪️</h3>
      </div>
      <div class="px-5">
        {#if tasks.length > 0}
          {#each tasks as task, index (index)}
            <div class="flex group flex-row items-center gap-3 mb-2">
              <a
                href={""}
                on:click|preventDefault={() => {
                  id = task.id;
                  completed = task.completed;
                  toggleComplete();
                }}
              >
                <span
                  class={`group-hover:text-black ${!task.completed ? "text-[#797979]" : "text-gray-400"} transition-all`}
                  >╰┈➤</span
                >
                {#if !task.completed}
                  <span class="font-bold inline">
                    {`🐣️ `}
                  </span>
                {:else}
                  <span class="font-bold inline">
                    {`🚀️ `}
                  </span>
                {/if}
                &nbsp;
                <div
                  class={`text-black group-hover:text-gray-800 inline-flex max-w-[410px] truncate group-hover:font-medium transition-all ease-in-out group-hover:tracking-tight ${task.completed ? `line-through text-gray-400` : ""}`}
                >
                  {task.description}
                </div>
              </a>

              {#if !task.deleting}
                <div
                  class="inline-flex opacity-0 group-hover:opacity-100 gap-2"
                >
                  <button
                    on:click={() => {
                      new_description = task.description;
                      id = task.id;
                      updating = true;
                    }}
                    class="border-2 text-gray-500 hover:text-white hover:hover:bg-gray-800 hover:border-gray-800 px-3 py-[2px] rounded-full font-medium transition-all"
                    >Edit</button
                  >
                  <button
                    on:click={() => toggleDeleting(task)}
                    class="border-2 text-gray-500 hover:text-white hover:hover:bg-gray-800 hover:border-gray-800 px-3 py-[2px] rounded-full font-medium transition-all"
                    >Delete</button
                  >
                </div>
              {:else}
                <div class="inline-flex gap-2 items-center">
                  <!-- <p class="inline">Delete task?</p> -->

                  <button
                    on:click={() => deleteTask(task.id)}
                    class="border-2 text-gray-500 hover:text-white hover:hover:bg-gray-800 hover:border-gray-800 px-3 py-[2px] rounded-full font-medium transition-all"
                    >Delete</button
                  >

                  <button
                    on:click={() => toggleDeleting(task)}
                    class="border-2 text-gray-500 hover:text-white hover:hover:bg-gray-800 hover:border-gray-800 px-3 py-[2px] rounded-full font-medium transition-all"
                    >Cancel</button
                  >
                </div>
              {/if}
            </div>
          {/each}
        {:else}
          <p>😭️ No task to show yet.</p>
        {/if}
      </div>
    </section>
  </div>
</main>
