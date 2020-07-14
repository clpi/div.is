<script>
  import { slide, fade } from 'svelte/transition'
  import { setContext, beforeUpdate, getContext, onMount } from 'svelte'
  import {writable, readable} from 'svelte/store'
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
    h2 {
        font-size: 2rem;
    }
    .home-wrapper {
        padding-left: 15.5vw;
        padding-right: 15.5vw;
        margin: auto;
        align-content: center;
        display: inline-block;
    }
    .box {
		width: 300px;
        float: left;
		border: 1px solid #fa4;
		border-radius: 2px;
		box-shadow: 2px 2px 8px rgba(0,0,0,0.1);
		padding: 1em;
		margin: 1em 1em 1em 1em;
	}
</style>

<div class="home-wrapper" in:fade={{duration:100}}>
    <div class="box">
        <h2>Hello</h2>
        <p>{userData}</p>
    </div>
    <div class="box">
        <h2>How are you</h2>
        <p>Look for <a href="/user">users</a></p>
    </div>
    <div class="box">
        <h2>try loggin</h2>
        <p>in <a href="/login">here</a>.</p>
    </div>    
    <div class="box">
        <h2>admin?</h2>
        <p>go to admin <a href="/admin">page</a>.</p>
    </div>    
</div>
