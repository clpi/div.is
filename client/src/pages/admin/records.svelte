<script>
  import { slide, fade } from 'svelte/transition'
  import Box from '../../comp/ui/box.svelte';
  import Tab from '../../comp/ui/tab.svelte';
</script>
<style>
</style>

<div in:fade={{duration:100}}>
    <h1>Records</h1>
        <Tab label = "Home">
        </Tab>
    <Tab label="Update">
    </Tab>
    <Tab label="Delete"></Tab>
</div>
