<script>
  import { BACKEND_BASE_URL } from "$lib/globals.js";
  import Fa from "svelte-fa/src/fa.svelte";
  import { faPlus } from "@fortawesome/free-solid-svg-icons";
  import { onMount } from "svelte";
  import Grade from "./grade.svelte";
  /** @type {import('./$types').PageData} */
  let data;
  onMount(async () => {
    data = await fetch(BACKEND_BASE_URL + "/subject?_embed=grade").then((r) =>
      r.json()
    );
  });
  async function newGrade(subject_id) {
    await fetch(BACKEND_BASE_URL + "/grade", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      referrerPolicy: "no-referrer",
      body: JSON.stringify({ grade: 4, subjectId: subject_id }),
    });
    data = await fetch(BACKEND_BASE_URL + "/subject?_embed=grade").then((r) =>
      r.json()
    );
  }
  function deleteGrade(id) {
    if (data != undefined) {
      console.log(data);
      data = data.map((subject) => {
        subject.grade = subject.grade.filter((grade) => grade.id != id);
        return subject;
      });
      console.log(data);
    }
  }
</script>

{#await data}
  <p>fetching grades...</p>
{:then data}
  {#if data != undefined}
    {#each data as subject (subject.id)}
      <div class="grades">
        {#each subject.grade as grade (grade.id)}
          <Grade
            grade={{ ...grade, subject: subject }}
            on:delete={() => deleteGrade(grade.id)}
          />
        {/each}
        <div style="display: flex;">
          <div style="flex-grow: 1;" />
          <button on:click={() => newGrade(subject.id)}>
            <Fa icon={faPlus} size="2em" />
          </button>
          <div style="flex-grow: 1;" />
        </div>
      </div>
    {/each}
  {/if}
{:catch error}
  <p>{error.message}</p>
{/await}

<style>
  .grades {
    display: flex;
    flex-direction: column;
  }

  button {
    display: block;
    background-color: #d7c7c0;
    max-width: 50%;
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
  .a {
    flex-grow: 1;
    background-color: red;
    padding: 1em;
  }
</style>
