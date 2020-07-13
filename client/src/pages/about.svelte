<script>
  import { onMount } from 'svelte';	
  import { slide, fade } from 'svelte/transition'
  onMount(async () => {
    await fetch('http://localhost:3001/api/sum/29/69')
      .then(res => res.text())
      .then(body => expression = body)
      .catch(err=>{
        console.log(err);
    });
    await fetch('http://localhost:3001/api/user/chrisp')
      .then(res=> res.json())
      .then(body => userInfo = body)
      .catch(err=>{
        console.log(err);
      });
  });
  let expression = "";
  let userInfo = "";
</script>

<style>
    .about {
        padding-left: 15.5vw;
        padding-right: 15.5vw;
    }
</style>
<div class="about" in:fade={{duration:100}}>
    <h1>About</h1>
    <p>Logged in as: <code>{userInfo.username}</code> ({userInfo.email})</p>
    <p>{expression}</p>
</div>
