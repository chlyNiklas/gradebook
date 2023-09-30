<script>
  import Fa from "svelte-fa/src/fa.svelte";
  import { faGraduationCap } from "@fortawesome/free-solid-svg-icons";
  import Modal from "./modal.svelte";
  export let elements = [
    { name: "Home", href: "/" },
    { name: "Home", href: "/" },
    { name: "Home", href: "/" },
    { name: "Home", href: "/" },
    { name: "Home", href: "/" },
    { name: "Home", href: "/" },
    { name: "Home", href: "/" },
    { name: "Home", href: "/" },
    { name: "Home", href: "/" },
    { name: "Home", href: "/" },
  ];
  let showMobile = false;
</script>

<nav>
  <h1>
    <Fa icon={faGraduationCap} /> Gradebook
  </h1>
  <div class="desktop">
    {#each elements as element}
      <a href={element.href}>{element.name}</a>
    {/each}
  </div>
  <div class="mobile">
    {#if showMobile}
      <Modal bind:showModal={showMobile}>
        {#each elements as element}
          <a href={element.href}>{element.name}</a>
        {/each}
      </Modal>
    {:else}
      <button on:click={() => (showMobile = true)}>
        <Fa icon={faGraduationCap} />
      </button>
    {/if}
  </div>
</nav>

<style>
  nav {
    max-width: 100vw;
    display: flex;
    flex-direction: row;
    justify-content: flex-start;
    align-items: center;

    background-color: lightgray;
    padding: 0 1em 0 1em;
  }
  h1 {
    flex-shrink: 0;
  }
  @media (min-width: 800px) {
    .mobile {
      display: none;
    }
    .desktop {
      display: flex;
      flex-direction: row;
      justify-content: flex-start;
      align-items: center;
      flex-wrap: wrap;
      flex-shrink: 1;
    }
  }
  @media (max-width: 800px) {
    .desktop {
      display: none;
    }
    .mobile {
      display: block;
    }
  }

  a {
    margin: 1em;
  }
</style>
