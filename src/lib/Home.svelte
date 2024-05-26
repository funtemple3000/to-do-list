<script>
  import { invoke } from "@tauri-apps/api/tauri"
  import { save, open, message } from "@tauri-apps/api/dialog";
  import { exists, readTextFile, writeBinaryFile, writeTextFile } from "@tauri-apps/api/fs";
  import { homeDir } from "@tauri-apps/api/path";
  import { createEventDispatcher } from "svelte";
  import { writeText } from "@tauri-apps/api/clipboard"

  const dispatch = createEventDispatcher();

  let action = "";
  let created = false;

  async function writeError(error){
    await writeText(error);
    await message("There was an error creating/opening the JSON file: " + error + "\nThe error message has automatically been copied to the clipboard.", { title: "Error", type: "error" });
  }

  async function createToDoList(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    //greetMsg = await invoke("greet", { name })

    if(action === "create"){
      try{
        const filePath = await save({
          defaultPath: await homeDir() + "myToDoList.json",
          filters: [{
            name: "JSON Files",
            extensions: ["json"]
          }],
          title: "Create New To Do List"
        });
      
        if(filePath){
          const jsonData = await invoke("create_new_template");
          const jsonString = JSON.stringify(jsonData, null, 2);

          await writeTextFile(filePath, jsonString, { append: false });
          created = true;

          dispatch("submit", { value: {jsonData: jsonData, filePath: filePath} });
        } else {
          await message("The dialog was cancelled and no file was written.", "Dialog Cancelled");
        }
      } catch(error){
        await writeError(error);
      }
      

    } else if(action === "open"){
      try{
        const selected = await open({
          defaultPath: await homeDir(),
          filters: [{
            name: "JSON Files",
            extensions: ["json"]
          }],
          title: "Open existing To Do List",
          multiple: false
        });

        if(typeof selected === "string"){
          if(await exists(selected)){
            const fileContent = await readTextFile(selected);
            const jsonData = JSON.parse(fileContent);
            dispatch("submit", { value: {jsonData: jsonData, filePath: selected} });
          } else {
            await message("File does not exist.", "Nonexistent File");
          }
          
        } else {
          await message("The dialog was cancelled and no file was written.", "Dialog Cancelled");

        }
      } catch(error){
        await writeError(error);
      }
    } else {
      await message("Please select one.", "Field Empty");
    }

  }
</script>

<div>
  <h3>I want to...</h3>
  <div class="row">
    <button on:click={() => { action = "create"; createToDoList(); }}>Create a new To Do List</button>
    <button on:click={() => { action = "open"; createToDoList(); }}>Open existing To Do List</button>
  </div>
  

</div>

