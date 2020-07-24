<script>
  import { params } from '@sveltech/routify';
  import { onMount } from 'svelte';

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
  <h1>{$params}</h1>  
</div>
