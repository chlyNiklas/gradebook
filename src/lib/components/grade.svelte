<script>
  import Fa from "svelte-fa/src/fa.svelte";
  import { faWrench } from "@fortawesome/free-solid-svg-icons";
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
</script>

<div>
  <p><strong>{grade.subject.name}:</strong> {grade.grade}</p>
  <button on:click={() => (editMode = true)}><Fa icon={faWrench} /></button>
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
</div>
