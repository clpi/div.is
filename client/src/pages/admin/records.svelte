<script>
  import { slide, fade } from 'svelte/transition'
  import Box from '../../comp/ui/box.svelte';
  let recordPostPromise = Promise.resolve([]);
  let submittedRecordPost = false;
  let recordPost = {
    uid: 0, //int
    name: "", //string
    status: "", //string
    permission: false, //bool
  }
  async function createRecord() {
    const post = await fetch(API_URL+'/record', {
      method: 'POST',
      headers: {
        'content-type': 'application/json',
      },
      body: JSON.stringify(recordPost)
    })
        .catch(err=>{
            console.log(err);
        });
    if (post.ok) {
        return post.json();
    } else {
      throw new Error(users);
    }
  }
  function handleCreateRecord() {
      recordPostPromise = createRecord();
      submittedRecordPost = true;
  }
</script>
<style>
    ul {
        list-style-type: none;
    }
    li {
    }
</style>

<div in:fade={{duration:100}}>
    <h1>Records</h1>
    <Box title="Post record">
        <form label="Username">
          <ul>
            <li>uid: <input label="uid" type=number bind:value={recordPost.uid}></li>
            <li>name: <input label="name" bind:value={recordPost.name}/></li>
            <li>status: <input label="status" bind:value={recordPost.status}/></li>
            <li>private: <input type=checkbox bind:checked={recordPost.permission}></li>
        </form>             
        <button on:click={handleCreateRecord}>Submit</button>
    </Box>
    {#if submittedRecordPost}
        {#await recordPostPromise}
            <p>Posting record...</p>
        {:then r}
            <li>
              <Box title="User">
                <p in:fade>uid:{ r.uid }</p>
                <p in:fade><b>name: </b>{r.name}</p>
                <p in:fade><b>status: </b>{r.status}</p>
                <p in:fade><em><b>Private</b> { r.permission }</em></p>
                <p in:fade><em><b>Created at</b> { r.createdAt }</em></p>
              </Box>
            </li>
        {:catch}
            <p>Couldnt create record</p>
        {/await}
    {/if}
</div>
