<script>
  import { slide, fade } from 'svelte/transition'
  import Box from '../../comp/ui/box.svelte';
  import Tab from '../../comp/ui/tab.svelte';
</script>

<style>
</style>

<div in:fade={{duration:100}}>
    <h1>Admin Dashboard</h1>
    <Box label="Admin">
        <p>Hello</p>
    </Box> 
        <button 
            type="submit" 
            on:click={ console.log("hello") }
            >
            Press
        </button>
</div>
