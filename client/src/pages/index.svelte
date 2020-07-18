<script>
  import { slide, fade } from 'svelte/transition'
  import { setContext, beforeUpdate, getContext, onMount } from 'svelte'
  import {writable, readable} from 'svelte/store'
  import Box from '../comp/ui/box.svelte';
  import { metatags } from '@sveltech/routify';
  metatags.title = "memri";
  export const session = writable(null);
  let loggedIn = getContext('loggedIn'); 
  let userData;
  beforeUpdate(async () => {
      if (loggedIn) {
          let res = await fetch('http://localhost:3001/api/userstatus', {
              method: 'GET',
              credentials: 'include',
              headers: {
                cookie: document.cookie,
              }
          });
          if (!res.ok) {
            setContext('loggedIn', false);
            setContext('userData', null);
            loggedIn = false;
          }
      }
    userData = getContext('userData');
  })
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
        <p>{userData}</p>
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
