<script>
  import { onMount } from "svelte";
  import { getAllNotes, postNote } from "src/api";

  
  let notes = [];
  onMount(() => {
    notes = getAllNotes();
  });

  let textarea;

  const saveNote = () => {
    postNote({
      content: textarea,
      created: new Date()
    })
    notes = getAllNotes()
  }

</script>

<style lang="scss">
  :global(body) {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100vh;
    margin: 0;
  }

  .main-container {
    width: min-content;
    height: min-content;
  }
</style>

<section class="main-container">
  <textarea name="note" cols="40" rows="5" bind:value={textarea} />
  <section>
    {#each notes as { content }}
      <p>{content}</p>
    {/each}
  </section>
  <button on:click={saveNote} > Save </button>
</section>
