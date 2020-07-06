
<script>
  import { slide, fade } from 'svelte/transition'
</script>
<style>
    h1 {
        font-size: 2rem;
        font-weight: 300;
    }
</style>
<div class="dbindex" in:fade={{duration:100}}>
    <h1>items</h1>
    <p>This is where records will go...</p>
</div>
