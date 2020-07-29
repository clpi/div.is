<script>
  import { slide, fade } from 'svelte/transition'
  import Box from '../../comp/ui/box.svelte';
  import Tab from '../../comp/ui/tab.svelte';
</script>
<style>
  .items {
    display: grid;
  } 
</style>

<div class="items" in:fade={{duration:100}}>
    <h1>Items</h1>
    <div class="item-left">
      <h3>My items</h3>
      <Box title="Item 1">
        <p>Here's item 1...</p>
      </Box>
    </div>
    <div class="item-right">
      <h3>New item</h3>
      <Box title="New item">
        <p>New item...</p>
      </Box>
    </div>

    <h3>Etc</h3>
</div>
