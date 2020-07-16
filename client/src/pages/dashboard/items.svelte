<script>
  import { slide, fade } from 'svelte/transition'
  import Box from '../../comp/ui/box.svelte';
  import Tab from '../../comp/ui/tab.svelte';
</script>
<style>
</style>

<div in:fade={{duration:100}}>
    <h1>Items</h1>
        <Tab label = "Home">
            <h2>Items Home<h2>
                <Box title="hello">
                <h3>hello</h3>
                </Box>
        </Tab>
    <Tab label="New">
    </Tab>
    <Tab label="Fields"></Tab>
</div>
