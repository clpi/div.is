<script>
  import { slide, fade } from 'svelte/transition'
  import Box from '../../comp/ui/box.svelte';
  import { createRecord, getUserRecords } from '../../util/calls';
  /*let recordPostPromise = createRecord();*/
  let uid = 1;
  function handleCreateRecord() {
      recordPostPromise = createRecord();
      submittedRecordPost = true;
  }
  function handleGetRecords() {
      recordPostPromise = getUserRecords(uid);
      submittedRecordPost = true;
  }
</script>
<style>
</style>

<div in:fade={{duration:100}}>
        <h1>Records</h1>
</div>
