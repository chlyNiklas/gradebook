<script>
  import Fa from "svelte-fa/src/fa.svelte";
  import { faClose } from "@fortawesome/free-solid-svg-icons";
  import { createEventDispatcher } from "svelte";
  export let showModal; // boolean
  const dispatch = createEventDispatcher();

  let dialog; // HTMLDialogElement

  $: if (dialog && showModal) dialog.showModal();
</script>

<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-noninteractive-element-interactions -->

<dialog
  bind:this={dialog}
  on:close={() => {
    showModal = false;
    dispatch("close");
  }}
  on:click|self={() => dialog.close()}
>
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div on:click|stopPropagation>
    <button autofocus on:click={() => dialog.close()}
      ><Fa icon={faClose} size="2em" /></button
    >
    <slot />
    <!-- svelte-ignore a11y-autofocus -->
  </div>
</dialog>

<style>
  button {
    position: absolute;
    top: 0.5em;
    right: 0.5em;

    padding: 0;
    border: none;
    background-color: #f2ece9;
  }
  dialog {
    min-width: 20vw;
    border: solid 1px #272524;
    border-radius: 1em;
    background-color: #f2ece9;
  }
</style>
