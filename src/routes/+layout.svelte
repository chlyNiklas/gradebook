<script>
  import Nav from "../lib/components/nav.svelte";
</script>

<Nav elements={[{name: "Home", href: "/"}]}/>
<main>
  <slot />
</main>

<style>
  main {
    padding: 1px;
  }
  @media (max-width: 400px) {
    main {
      margin: 0 ;
    }
  }
  @media (min-width: 400px) and (max-width: 800px) {
    main {
      margin: 0 15vw ;
    }
  }
  @media (min-width: 800px) {
    main {
      margin: 0 25vw 0 25vw;
    }
  }
</style>
