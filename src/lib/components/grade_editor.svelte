<script>
  import { BACKEND_BASE_URL } from "$lib/globals.js";
  import { onMount } from "svelte";
  import Grade from "./grade.svelte";
  import PlusButon from "./atoms/plus_buton.svelte";
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
    <div class="subjects">
      {#each data as subject (subject.id)}
        <div class="grades">
          <h1>{subject.name}:</h1>
          {#each subject.grade as grade (grade.id)}
            <Grade
              grade={{ ...grade, subject: subject }}
              on:delete={() => deleteGrade(grade.id)}
            />
          {/each}
          <div style="display: flex;">
            <div style="flex-grow: 1;" />
            <PlusButon on:click={() => newGrade(subject.id)} />
            <div style="flex-grow: 1;" />
          </div>
        </div>
      {/each}
    </div>
  {/if}
{:catch error}
  <p>{error.message}</p>
{/await}

<style>
  .grades {
    display: flex;
    flex-direction: column;
  }
  .subjects {
    display: grid;
    gap: 1em;
  }
  @media only screen and (max-width: 900px) {
    .subjects {
      grid-template-columns: repeat(1, 1fr);
    }
  }
  @media only screen and (min-width: 900px) and (max-width: 1200px) {
    .subjects {
      grid-template-columns: repeat(2, 1fr);
    }
  }
  @media only screen and (min-width: 1200px) {
    .subjects {
      grid-template-columns: repeat(3, 1fr);
    }
  }
</style>
