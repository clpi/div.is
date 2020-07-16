<script>
  import { slide, fade } from 'svelte/transition'
  import Box from '../../comp/ui/box.svelte';
  import Tab from '../../comp/ui/tab.svelte';
</script>
<style>
</style>

<div in:fade={{duration:100}}>
    <h1>Entries</h1>
        <Tab label = "Entries">
            <h2>Entries Home<h2>
                    <Box title="entries">dsf</Box>
        </Tab>
    <Tab label="Types">
    </Tab>
    <Tab label="Log"></Tab>
</div>
