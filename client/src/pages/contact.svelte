<script>
  import { slide, fade } from 'svelte/transition'
  import { metatags } from '@sveltech/routify';
  metatags.title = "contact";
</script>

<style>
    .contact {
    }
</style>

<div class="contact" in:fade={{duration:100}}>
    <h1>Contact</h1>
</div>
