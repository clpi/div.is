<script>
  import { params } from '@sveltech/routify';
  export let slug;
</script>

<style>
  .user {
    padding-left: 3vw;
  }
</style>

<div class = "user">
  <h1>{$params}</h1>  
</div>
