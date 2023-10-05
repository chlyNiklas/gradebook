<script>
  import { BACKEND_BASE_URL } from "$lib/globals.js";
  import PlusButon from "./atoms/plus_buton.svelte";
  import { onMount } from "svelte";
  let data;
  onMount(async () => {
    data = await fetch(BACKEND_BASE_URL + "/subject").then((r) => r.json());
  });
</script>

{#await data}
  <p>fetching grades...</p>
{:then data}
  {#if data != undefined}
    <div class="subjects">
      {#each data as subject (subject.id)}
        <div class="subject container">
          <h1>{subject.name}</h1>
        </div>
      {/each}
    </div>
    <PlusButon />
  {/if}
{:catch error}
  <p>{error.message}</p>
{/await}
