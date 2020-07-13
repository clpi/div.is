<script>
  import { slide, fade } from 'svelte/transition'
  import { Button } from 'svelma'
</script>

<style>
    .admin-index {
        padding-left: 15.5vw;
        padding-right: 15.5vw;
    }
   h1 {
        font-size: 2.2rem;
        font-weight: 300;
    }
</style>

<div class="admin-index" in:fade={{duration:100}}>
    <h1>Admin Dashboard</h1>
        <Button 
            type="is-primary" 
            nativeType="submit" 
            on:click={ console.log("hello") }
            >
            Press
        </Button>
</div>
