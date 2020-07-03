<script>
  import Hero from '../comp/hero.svelte'
  import { slide, fade } from 'svelte/transition'
</script>

<style>
    .contact {
        padding-left: 15.5vw;
        padding-right: 15.5vw;
    }
</style>

<div class="contact" in:fade={{duration:100}}>
    <h1>About</h1>
</div>
