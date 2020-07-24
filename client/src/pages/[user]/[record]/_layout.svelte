<script>
  import { params } from '@sveltech/routify';
  import { onMount } from 'svelte';

  onMount(async () => {
    console.log($params)
  });

  console.log($params);
</script>

<style>
  .record {
  }

</style>

<div class = "record">

</div>

