<script>
  import { slide, fade } from 'svelte/transition'
  import { setContext, beforeUpdate, getContext, onMount } from 'svelte'
  import {writable, readable} from 'svelte/store'
  import Box from '../comp/ui/box.svelte';
  import { metatags } from '@sveltech/routify';
  metatags.title = "div.is";
  export let scoped;
</script>

<style>
    .indH1 {
        font-size: 6rem;
    }
    .home-wrapper {
        margin: auto;
        align-content: center;
        display: inline-block;
    }
</style>

<div class="home-wrapper" in:fade={{duration:100}}>
    <Box title={"Hello"}>
      <p>Welcome, {scoped.user.username}</p>
    </Box>
    <Box title={"How are you"}>
        <p>Look for <a href="/user">users</a></p>
    </Box>
    <Box title={"Try logging in"}>
        <p>in <a href="/login">here</a>.</p>
    </Box>    
    <Box title={"Admin?"}>
        <p>go to admin <a href="/admin">page</a>.</p>
    </Box>    
</div>
