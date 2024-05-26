<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import { message, confirm } from "@tauri-apps/api/dialog";
    import { writeText } from "@tauri-apps/api/clipboard";
    import { writeTextFile } from "@tauri-apps/api/fs";

    export let file = { to_do_list: [] };
    export let filePath = null;

    const iconSize = 40;

    let isCreatingNewTask = false;

    let newTaskName = "";

    let index = 0;

    let state = false;

    async function addTaskBtn() {
        isCreatingNewTask = true;
    }
    async function addTask(){
        try{
            const result = await invoke("add_task", { file: file, taskName: newTaskName });
            file = result;
            await writeFile();
            
        } catch (error) {
            writeError(error);
            
        } finally {
            isCreatingNewTask = false;
            newTaskName = "";
        }
        
    }

    async function moveUp(){
        try{
            const result = await invoke("move_up", { file: file, index: index });
            file = result;
            await writeFile();
        } catch (error) {
            await writeError(error);
            
        }
    }
    async function moveDown(){
        try{
            const result = await invoke("move_down", { file: file, index: index });
            file = result;
            await writeFile();
        } catch (error) {
            await writeError(error);
            
        }
    }
    async function deleteTask(){
        if(!(await confirm("Are you sure you want to delete this task? This cannot be undone.", { title: "Confirm Deletion", type: "warning"}))){
            return;
        }
        try{
            const result = await invoke("delete_task", { file: file, index: index });
            file = result;
            await writeFile();
        } catch (error) {
            await writeError(error);
            
        }
    }

    async function changeState(){
        try{
            const result = await invoke("change_state", { file: file, index: index, state: state});
            file = result;
            await writeFile();
        } catch(error){
            await writeError(error);
        }
    }

    async function editTask(task, index){
        
        try{
            const result = await invoke("edit_task", { file: file, index: index, newTaskName: task.new_task_name});
            file = result;
            await writeFile();
        } catch (error){
            await writeError(error);
        }
    }

    async function deleteAllCompletedTasks(){
        if(await confirm("Are you sure you want to delete all completed tasks from the list? This action cannot be undone.", { title: "Confirm Deletion", type: "warning"})){
            try{
                const result = await invoke("delete_all_completed_tasks", { file: file });
                file = result;
                await writeFile();
            } catch (error) {
                await writeError(error);
            }
        }
    }

    async function writeError(error){
        await writeText(error);
        await message("An error occured while editing the list: " + error + "\nThe error message has automatically been copied to the clipboard.", { title: "Error", type: "error" });
    }

    async function writeFile(){
        if(filePath){
            await writeTextFile(filePath, JSON.stringify(file, null, 2), {append: false});
        }
    }
</script>


<div class="btn-container">
    
    <button on:click={addTaskBtn}>Add New Task</button>
    <button class="deleteBtn" on:click={deleteAllCompletedTasks}>Delete all completed tasks</button>
    {#if isCreatingNewTask}
        <div>
            
            <input class="textInput" placeholder="Enter task name..." bind:value={newTaskName}>
            <button on:click={addTask} class="greenBtn">Add</button>
            
        </div>
    {/if}
    {#each file.to_do_list as task, i}
        
        <div class="row" id="toDoList">
            <div class="checkbox-wrapper-31">
                <input id="checkbox" type="checkbox" checked={task.finished} on:change={() => {state = !task.finished; index = i; changeState();}}>
                <svg viewBox="0 0 35.6 35.6">
                    <circle class="background" cx="17.8" cy="17.8" r="17.8"></circle>
                    <circle class="stroke" cx="17.8" cy="17.8" r="14.37"></circle>
                    <polyline class="check" points="11.78 18.12 15.55 22.23 25.17 12.87"></polyline>
                  </svg>

                  
            </div>
            
            {#if task.is_editing}
                <div class="row">
                    <input class="textInput" placeholder={task.task_name} bind:value={task.new_task_name}>
                    <button class="greenBtn" on:click={() => { editTask(task, i);}}><svg xmlns="http://www.w3.org/2000/svg" width={iconSize} height={iconSize} fill="currentColor" class="bi bi-floppy-fill" viewBox="0 0 16 16">
                        <path d="M0 1.5A1.5 1.5 0 0 1 1.5 0H3v5.5A1.5 1.5 0 0 0 4.5 7h7A1.5 1.5 0 0 0 13 5.5V0h.086a1.5 1.5 0 0 1 1.06.44l1.415 1.414A1.5 1.5 0 0 1 16 2.914V14.5a1.5 1.5 0 0 1-1.5 1.5H14v-5.5A1.5 1.5 0 0 0 12.5 9h-9A1.5 1.5 0 0 0 2 10.5V16h-.5A1.5 1.5 0 0 1 0 14.5z"/>
                        <path d="M3 16h10v-5.5a.5.5 0 0 0-.5-.5h-9a.5.5 0 0 0-.5.5zm9-16H4v5.5a.5.5 0 0 0 .5.5h7a.5.5 0 0 0 .5-.5zM9 1h2v4H9z"/>
                      </svg></button>
                </div>
            {:else}
                {#if task.finished}
                    <p id="finishedTaskName"><s>{task.task_name}</s></p>
                {:else}
                    <p id="taskName">{task.task_name}</p>
                {/if}
                <button class="optionBtn" id="editBtn" on:click={() => { task.is_editing = true; }}><svg xmlns="http://www.w3.org/2000/svg" width={iconSize} height={iconSize} fill="currentColor" class="bi bi-pen-fill" viewBox="0 0 16 16">
                    <path d="m13.498.795.149-.149a1.207 1.207 0 1 1 1.707 1.708l-.149.148a1.5 1.5 0 0 1-.059 2.059L4.854 14.854a.5.5 0 0 1-.233.131l-4 1a.5.5 0 0 1-.606-.606l1-4a.5.5 0 0 1 .131-.232l9.642-9.642a.5.5 0 0 0-.642.056L6.854 4.854a.5.5 0 1 1-.708-.708L9.44.854A1.5 1.5 0 0 1 11.5.796a1.5 1.5 0 0 1 1.998-.001"/>
                  </svg></button>
            {/if}
            
            <button class="optionBtn" on:click={() => {index = i; moveUp();}}><svg xmlns="http://www.w3.org/2000/svg" width={iconSize} height={iconSize} fill="currentColor" class="bi bi-arrow-up" viewBox="0 0 16 16">
                <path fill-rule="evenodd" d="M8 15a.5.5 0 0 0 .5-.5V2.707l3.146 3.147a.5.5 0 0 0 .708-.708l-4-4a.5.5 0 0 0-.708 0l-4 4a.5.5 0 1 0 .708.708L7.5 2.707V14.5a.5.5 0 0 0 .5.5"/>
              </svg></button>
            <button class="optionBtn" on:click={() => {index = i; moveDown();}}><svg xmlns="http://www.w3.org/2000/svg" width={iconSize} height={iconSize} fill="currentColor" class="bi bi-arrow-down" viewBox="0 0 16 16">
                <path fill-rule="evenodd" d="M8 1a.5.5 0 0 1 .5.5v11.793l3.146-3.147a.5.5 0 0 1 .708.708l-4 4a.5.5 0 0 1-.708 0l-4-4a.5.5 0 0 1 .708-.708L7.5 13.293V1.5A.5.5 0 0 1 8 1"/>
              </svg></button>
            <button class="optionBtn deleteBtn" on:click={() => {index = i; deleteTask();}}>
                <svg xmlns="http://www.w3.org/2000/svg" width={iconSize} height={iconSize} fill="currentColor" class="bi bi-trash" viewBox="0 0 16 16">
                    <path d="M5.5 5.5A.5.5 0 0 1 6 6v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5m2.5 0a.5.5 0 0 1 .5.5v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5m3 .5a.5.5 0 0 0-1 0v6a.5.5 0 0 0 1 0z"/>
                    <path d="M14.5 3a1 1 0 0 1-1 1H13v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V4h-.5a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1H6a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1h3.5a1 1 0 0 1 1 1zM4.118 4 4 4.059V13a1 1 0 0 0 1 1h6a1 1 0 0 0 1-1V4.059L11.882 4zM2.5 3h11V2h-11z"/>
                  </svg>
            </button>
        </div>
    {/each}
    
</div>

