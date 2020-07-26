<script>
  import { params } from '@sveltech/routify';
  import { onMount } from 'svelte';
  
  export let user;

  onMount(async () => {
    console.log($params)
  });
</script>

<style>
  .user {
    padding-left: 3vw;
  }
</style>

<div class = "user">
  <h1>user</h1>
</div>
