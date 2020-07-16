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
            <h2>Records Home<h2>
                    <Box title="hello">fddsf</Box>
        </Tab>
    <Tab label="New">
    </Tab>
    <Tab label="Items"></Tab>
</div>
