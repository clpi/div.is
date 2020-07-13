<script>
  import { slide, fade } from 'svelte/transition'
  import { setContext, getContext, onMount } from 'svelte'
  let loggedIn;
  let userData;
  onMount(async () => {
      let res = await fetch('http://localhost:3001/api/userstatus');
      if (res.ok) {
        setContext(loggedIn, true);
        setContext(userData, res.json());
      } else {
        /*setContext(loggedIn, false);*/
        /*setContext(userData, null);*/
      }
      console.log(res.json())
      loggedIn = getContext(loggedIn);
      userData = getContext(userData);
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
        <p>{ loggedIn } { userData }</p>
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
