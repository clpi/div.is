<script>
  import { onMount } from 'svelte';	
  import { slide, fade } from 'svelte/transition'
  import { metatags } from '@sveltech/routify';
  metatags.title = "about";
  onMount(async () => {
    await fetch(API_URL+'/sum/29/69')
      .then(res => res.text())
      .then(body => expression = body)
      .catch(err=>{
        console.log(err);
    });
    await fetch(API_URL+'/user/chrisp')
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
</style>
<div class="about" in:fade={{duration:100}}>
    <h1>About</h1>
    <p>Logged in as: <code>{userInfo.username}</code> ({userInfo.email})</p>
    <p>{expression}</p>
</div>
