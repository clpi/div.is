<script>
  import { slide, fade } from 'svelte/transition'
</script>

<style>
    .admin-index {
        padding-left: 15.5vw;
        padding-right: 15.5vw;
    }
</style>

<div class="admin-index" in:fade={{duration:100}}>
    <h1>Admin Dashboard</h1>
</div>
