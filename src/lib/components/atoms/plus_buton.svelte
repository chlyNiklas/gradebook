<script>
  import Fa from "svelte-fa/src/fa.svelte";
  import { faPlus } from "@fortawesome/free-solid-svg-icons";
</script>

<button on:click>
  <Fa icon={faPlus} size="2em" />
</button>

<style>
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
</style>
