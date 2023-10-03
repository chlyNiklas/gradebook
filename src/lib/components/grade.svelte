<script>
  import { BACKEND_BASE_URL } from "$lib/globals.js";
  import Fa from "svelte-fa/src/fa.svelte";
  import Modal from "./modal.svelte";
  import { createEventDispatcher } from "svelte";
  import { faTrash } from "@fortawesome/free-solid-svg-icons";

  const dispatch = createEventDispatcher();
  export let grade;
  let editMode = false;
  let deleteMode = false;

  async function update() {
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
    dispatch("update", response);
  }
  async function del() {
    const response = await fetch(BACKEND_BASE_URL + "/grade/" + grade.id, {
      method: "DELETE", // *GET, POST, PUT, DELETE, etc.
      headers: {
        "Content-Type": "application/json",
      },
    });
    dispatch("delete", response);
    editMode = false;
  }
</script>

<div
  class:bad={grade.grade < 4}
  class:enough={grade.grade >= 4 && grade.grade < 5}
  class:good={grade.grade >= 5}
>
  <button
    style="flex-grow: 1;"
    on:click={() => (editMode = true)}
    class="grade"
    class:bad={grade.grade < 4}
    class:enough={grade.grade >= 4 && grade.grade < 5}
    class:good={grade.grade >= 5}
  >
    <p>
      <strong>{grade.subject.name}:</strong>
      {grade.grade}
    </p>
  </button>
  <Modal bind:showModal={editMode} on:close={update}
    ><h1>Edit</h1>
    <input type="number" min="1" max="6" step="0.25" bind:value={grade.grade} />
  </Modal>
  <button
    style="flex-grow:0;"
    class="trash"
    on:click={() => (deleteMode = true)}
    class:bad={grade.grade < 4}
    class:enough={grade.grade >= 4 && grade.grade < 5}
    class:good={grade.grade >= 5}><Fa icon={faTrash} /></button
  >
  <Modal bind:showModal={deleteMode}
    ><h1>Are you shure?</h1>
    <button on:click={del}>Yes</button>
    <button on:click={() => (deleteMode = false)}>No</button>
  </Modal>
</div>

<style>
  div {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
    margin: 1em;
    border: solid 1px;
    border-radius: 1em;
    border-color: #272524;
    padding: 1em;
    box-shadow: 0 0 10px #27252430;
    transition: 0.1s;
    flex-grow: 1;
  }
  div:hover {
    box-shadow: 0 0 10px #27252480;
    transition: 0.1s;
  }
  button {
    box-shadow: 0 0 0 #0000;
    width: auto;
    border: 0;
  }
  button,
  button * {
    margin: 1em;
  }
  p {
    margin: 0;
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
  .modal button {
    margin: 1em;
  }
</style>
