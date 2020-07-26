<script>
  import { params } from '@sveltech/routify';
  import { onMount } from 'svelte';
  /*import { page } from '@sveltech/routiy';*/
  import { metatags } from '@sveltech/routify'
  import { user, logged } from '../../store.js';
  /*metatags.title = 'User';*/
  
  /*export let usr;*/
  /*export let log;*/

  onMount(async () => {
    console.log($params.props)
  });
</script>

<style>
  .user {
    padding-left: 3vw;
  }
  .wrapper {
    padding-left: 5vw;
  }
  .user {
    margin-top: 40px;
    background-color: #fefdfd;
    border: 2px solid #eee;
    border-bottom-color: #fd9;
    border-bottom-width: 3px;
    border-radius: 8px;
    padding-top: 5%;
    padding-bottom: 10%;
    align-items: center;
    padding-left: 15.5vw;
    padding-right: 15.5vw;
    box-shadow: 1px 2px 2px rgba(0,0,0,0.1);
  }
  .user-head {
    display: inline-block;
    margin: auto;
    text-align: center;
  }
  .title {
    font-weight: 300;
    text-shadow: -0px -1px 1px #fff;
    font-size: 2.5rem;
  }
  h1 { 
      font-size: 2rem;
      font-weight: 300;
      text-shadow: 0px 1px 1px #444;
  }
  h2 {
      font-size: 1.6rem;
      font-weight: 300;
  }
  h3 {
      font-size: 1.3rem;
      font-weight: 300;
  }
</style>

<div class = "wrapper">
  <div class = "user">
    <div class = "user-head">
      <h1 class="title">Hello!</h1>
    </div>
  </div>
</div>
