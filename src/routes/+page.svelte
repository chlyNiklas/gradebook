<script>
  import Grade from "$lib/components/grade.svelte";

  /** @type {import('./$types').PageLoad} */
  export let data;
</script>

{#await data}
  <p>fetching grades...</p>
{:then data}
  {#each data.data as subject}
    <div class="grades">
      {#each subject.grade as grade}
        <Grade grade={{ ...grade, subject: subject }} />
      {/each}
    </div>
  {/each}
{:catch error}
  <p>{error.message}</p>
{/await}
  <style>
  .grades {
    display: flex;
    flex-direction: column;
  }
</style>
