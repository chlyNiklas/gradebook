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
    console.log(editMode)
  }
</script>

<button
  on:click={() => (editMode = true)}
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
    background-color:white;
    margin: 1em;
    border: solid 1px;
    border-radius: 1em;
    border-color: black;
    padding: 1em;
    box-shadow: 0 0 10px #0003;
  }
  button:hover {
    box-shadow: 0 0 10px #0009;
  }
</style>
