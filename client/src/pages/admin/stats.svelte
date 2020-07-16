<script>
  import { slide, fade } from 'svelte/transition'
  import Box from '../../comp/ui/box.svelte';
  import Tab from '../../comp/ui/tab.svelte';
</script>
<style>
</style>

<div in:fade={{duration:100}}>
    <h1>Statistics</h1>
        <Tab label = "Users">
        </Tab>
    <Tab label="Traffic">
    </Tab>
    <Tab label="Performance"></Tab>
</div>

