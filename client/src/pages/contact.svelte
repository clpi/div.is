<script>
  import { slide, fade } from 'svelte/transition'
  import { metatags } from '@sveltech/routify';
  metatags.title = "contact";
  export let usr;
  export let log;
</script>

<style>
  .contact {
    padding-left: 3vw;
    text-align: center;
  }
  .contact-head {
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
<div class="contact" in:fade={{duration:100}}>
  <div class="contact-head">
    <h1 class="title" in:fade={{duration: 1200}}>Reach out </h1>
    <h1 class="title-2" in:fade={{duration: 1200, delay: 500}}>
      &nbsp;&nbsp;to me!</h1>
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
