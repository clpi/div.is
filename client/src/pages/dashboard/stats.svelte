<script>
  import { slide, fade } from 'svelte/transition'
  import Box from '../../comp/ui/box.svelte';
  import Tab from '../../comp/ui/tab.svelte';
</script>
<style>
</style>

<div in:fade={{duration:100}}>
    <h1>Stats</h1>
        <Tab label = "Home">
            <h2>Stats Dashboard<h2>
                    <Box title="stats">dsf</Box>
        </Tab>
    <Tab label="Records">
    </Tab>
    <Tab label="Entries"></Tab>
</div>
