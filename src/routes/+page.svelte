<script>
  /** @type {import('./$types').PageLoad} */
  export let data;
  console.log(data);
  $: {
    console.log(data);
  }
</script>

<h1>Welcome to SvelteKit</h1>
<p>
  Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
</p>
{#await data}
  <p>fetching grades...</p>
{:then data}
  {#each data.data as subject}
    <lu>
      {#each subject.grade as grade}
        <li>{subject.name}: {grade.grade}</li>
      {/each}
    </lu>
  {/each}
{:catch error}
  <p>{error.message}</p>
{/await}
