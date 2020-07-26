<script>
  import { onMount } from 'svelte';	
  import { slide, fade } from 'svelte/transition'
  import { metatags } from '@sveltech/routify';
  import { afterPageLoad } from "@sveltech/routify"
  metatags.title = "about";
  export let usr;
  export let log;
  $afterPageLoad(page => {
    console.log('loaded ' + page.title)
  })
</script>

<style>
  .about {
    padding-left: 3vw;
    text-align: center;
  }
  .about-head {
    display: inline-block;
    margin: auto;
    text-align: center;
  }
  .title {
    font-weight: 200;
    text-shadow: -0px -1px 1px #fff;
    font-size: 5.5rem;
    float: left;
  }
  .title-2 {
    float: left;
    font-weight: 300;
    font-size: 5.5rem;
  }
</style>
<div class="about" in:fade={{duration:100}}>
  <div class="about-head">
    <h1 class="title" in:fade={{duration: 1200}}>About </h1>
    <h1 class="title-2" in:fade={{duration: 1200, delay: 500}}>
      &nbsp;&nbsp;Our Project</h1>
  </div>
  <p> { usr } and { log } </p>
  {#await log}
  {:then}
    {#if log}
      <p>Welcome {usr.username}.</p>
    {:else}
      <p>Welcome, newcomer.</p>
    {/if}
  {/await}
</div>
