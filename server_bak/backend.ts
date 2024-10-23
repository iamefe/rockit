export async function getTasks() {
    const response = await fetch ("http://localhost:1420/api/get_tasks");
    return await response.json();
}

export async function addTask(description: String) {
    const response = await fetch("http://localhost:1420/api/add_task", {
        method: "POST",
        headers: { "Content-Type": "application/json"},
        body: JSON.stringify({ description })
    })
}