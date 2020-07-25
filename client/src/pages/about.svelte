<script>
  import { onMount } from 'svelte';	
  import { slide, fade } from 'svelte/transition'
  import { metatags } from '@sveltech/routify';
  import { afterPageLoad } from "@sveltech/routify"
  metatags.title = "about";
  $afterPageLoad(page => {
    console.log('loaded ' + page.title)
  })
</script>

<style>
  .about {
    padding-left: 3vw;
  }
</style>
<div class="about" in:fade={{duration:100}}>
    <h1>About</h1>
</div>
