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

<style>
    .title {
        font-size: 4em;
        text-shadow: 0px 2px 2px #555;
        font-weight:400;
        
    }
    .subtitle {
        font-size:2.2em;
        text-shadow: 0px 1.5px 1.5px #555;
        font-weight: 500;
    }
    .hero{
        background-image: linear-gradient(170deg, #ef8958, #ffdeaa);
        height: 15rem;
        color: #221;
        border-bottom: 4px solid rgba(255, 140, 100, 40)
    }
</style>

<section class="hero is-success">
    <div class="hero-body">
      <div class="container">
        <h1 class="title">
          memuri
        </h1>
        <h2 class="subtitle">
          Logging website
        </h2>
      </div>
    </div>
  </section>
<p>Logged in as: <code>{userInfo.username}</code> ({userInfo.email})</p>
<p>{x} + {y} gives: {expression}</p>
