<script>
  import { BACKEND_BASE_URL } from "$lib/globals.js";
  import Modal from "./modal.svelte";
  export let grade;
  let editMode = false;
  async function save() {
    const response = await fetch(BACKEND_BASE_URL + "/grade/" + grade.id, {
      method: "PUT", // *GET, POST, PUT, DELETE, etc.
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        id: grade.id,
        grade: grade.grade,
        subjectId: grade.subject.id,
      }), // body data type must match "Content-Type" header
    });
  }
  $: {
    console.log(editMode);
  }
</script>

<button
  on:click={() => (editMode = true)}
  class:bad={grade.grade < 4}
  class:enough={grade.grade >= 4 && grade.grade < 5}
  class:good={grade.grade >= 5}
>
  <p><strong>{grade.subject.name}:</strong> {grade.grade}</p>
  <Modal bind:showModal={editMode} on:click={save}
    ><h1>Edit</h1>
    <input
      type="number"
      min="1"
      max="6"
      step="0.25"
      bind:value={grade.grade}
    /></Modal
  >
</button>

<style>
  button {
    display: block;
    background-color: #f2ece9;
    margin: 1em;
    border: solid 1px;
    border-radius: 1em;
    border-color: #272524;
    padding: 1em;
    box-shadow: 0 0 10px #27252430;
    transition: 0.1s;
  }
  button:hover {
    box-shadow: 0 0 10px #27252480;
    transition: 0.1s;
  }
  .bad {
    background-color: #cd9f83;
  }
  .enough {
    background-color: #ebd8a3;
  }
  .good {
    background-color: #95ae91;
  }
</style>
