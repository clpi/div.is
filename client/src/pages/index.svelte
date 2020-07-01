<script>
  import { onMount } from 'svelte';	
  let x = 1;
  let y = 2;
  let expression = "";
  let userInfo = "";
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
</script>


<p>Logged in as: <code>{userInfo.username}</code> ({userInfo.email})</p>
<p>{x} + {y} gives: {expression}</p>
