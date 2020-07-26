<script>
  import { slide, fade } from 'svelte/transition'
  import { setContext, beforeUpdate, getContext, onMount } from 'svelte'
  import { user, logged } from '../store.js';
  import Box from '../comp/ui/box.svelte';
  import { metatags } from '@sveltech/routify';
  export let usr;
  export let log;
  metatags.title = "div.is";
  /*export let scoped;*/
</script>

<style>
  .home-wrapper {
    padding-left: 3vw;
    text-align: center;
    align-items: center;
    align-content: center;
  }
  .home-head {
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
  .subtitle {
    padding-bottom: 2.5rem;
    font-weight: 300;
    font-size: 2.5rem;
    color: #887978;
    margin-top: -30px;
    text-shadow: 0px -1px 1px #fff;
  }
  .home-wrapper {
      margin: auto;
      align-content: center;
  }
  .boxes {
    align-items: center;
    align-self: center;
    display: inline-block;
    width: 80%;
  }
  .box {
    display: inline-block;
    margin: 30%;
    width: 100%;
  }
</style>

<div class="home-wrapper" in:fade={{duration:100}}>
  <div class="home-head">
    <h1 class="title" in:fade={{duration: 1200}}>Welcome </h1>
    <h1 class="title-2" in:fade={{duration: 1200, delay: 500}}>
      &nbsp;&nbsp;to div.is</h1>
  </div>
  <h3 class="subtitle" in:fade={{duration: 800, delay: 1300}}> Start connecting the dots in your life. </h3>
  <div class="boxes" in:fade={{duration: 500, delay: 1700}}>
    <Box class="box" title={"Hello"}>
      {#if $user.username != undefined}
        <p>sup {$user.username}</p>
      {:else}
        <p>Not logged in</p>
      {/if}
      <!-- 
        <p>Welcome, {scoped.user.username}</p> 
      -->
    </Box>
    <Box class="box" title={"How are you"}>
        <p>Look for <a href="/user">users</a></p>
    </Box>
    <Box class="box" title={"Try logging in"}>
        <p>in <a href="/login">here</a>.</p>
    </Box>    
    <Box class="box" title={"Admin?"}>
        <p>go to admin <a href="/admin">page</a>.</p>
    </Box>    
    <Box class="box" title={"Admin?"}>
        <p>go to admin <a href="/admin">page</a>.</p>
    </Box>    
    <Box class="box" title={"Admin?"}>
        <p>go to admin <a href="/admin">page</a>.</p>
    </Box>    
  </div>
</div>
