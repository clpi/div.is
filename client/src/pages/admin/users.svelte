<script>
  import { slide, fade } from 'svelte/transition';
  import Box from '../../comp/ui/box.svelte';
  import Tab from '../../comp/ui/tab.svelte';
  import Infobox from '../../comp/ui/box.svelte';
  let userList;
  let promise = Promise.resolve([]);
  let submitted = false;
  let submittedUser = false;
  let userPromise = Promise.resolve([]);
  let userUsername = ""; let userId = ""; let userEmail = "";
  async function fetchAll() {
      const usrs = await fetch('http://localhost:3001/api/users', {
          method: 'GET',
          headers: {
            'content-type': 'application/json',
          },
      })
          .catch(err=>{
              console.log(err);
          });
      if (usrs.ok) {
          userList = usrs;
          return usrs.json();
      } else {
        throw new Error(users);
      }
    }
  async function getByEmail() {}
  async function getByUsername() {
      const usr = await fetch('http://localhost:3001/api/user/'+userUsername, {
          method: 'GET',
          headers: {
            'content-type': 'application/json',
          },
      })
          .catch(err=>{
              console.log(err);
          });
    if (usr.ok) { return usr.json() }
  }
  async function getById() {
      const usr = await fetch('http://localhost:3001/api/user/id'+userId, {
          method: 'GET',
          headers: {
            'content-type': 'application/json',
          },
      })
          .catch(err=>{
              console.log(err);
          });
    if (usr.ok) { return usr.json() }
  }
  function handleUsername() {
      userPromise = getByUsername();
      submittedUser = true;
  }
  function handleEmail() {
      userPromise = getByEmail();
      submittedUser = true;
  }
  function handleId() {
      userPromise = getById();
      submittedUser = true;
  }
  function fetchUsers() {
    promise = fetchAll();
    submitted = true;
  }

</script>
<style>
    ul {
        list-style-type: none;
        display: inline-block;
    }
    li {
        float: left;
    }
</style>

<div in:fade={{duration:100}}>
    <h1>User Admininistration</h1>
        <ul>
            <li>
                <Box title="Get user">
                    <form label="Username">
                        Username: <input label="Username" bind:value={userUsername}/>
                    </form>             
                    <button on:click={handleUsername}>Submit</button>
                </Box>
                <Box title="Get user">
                    <form label="Email">
                        Email: <input label="Email" bind:value={userEmail}/>
                    </form>             
                    <button on:click={handleEmail}>Submit</button>
                </Box>
                <Box title="By ID">
                    <form label="ID">
                        ID: <input label="ID" bind:value={userId}/>
                    </form>             
                    <button on:click={handleId}>Submit</button>
                </Box>
            </li>
            {#if submittedUser}
                {#await userPromise}
                    <p>Getting user...</p>
                {:then user}
                    <li>
                        <Box title="User">
                            <p in:fade>{ user.username }</p>
                            <p in:fade><b>id: </b>{user.id}</p>
                            <p in:fade><b>Email: </b>{user.email}</p>
                            <p in:fade><em><b>Created at</b> { user.created_at }</em></p>
                        </Box>
                    </li>
                {:catch}
                    <p>No user found.</p>
                {/await}
            {/if}
        </ul>
            <div>
                <button 
                    type="submit" 
                    on:click={ fetchUsers }
                >
                    Fetch
                </button>
            </div>
            <br/>
        {#if submitted}
            {#await promise}
                <p>Fetching users...</p>
            {:then users}
                <div>
                    <ul>
                        {#each users as user}
                            <li>
                                <Box title="User">
                                    <h3 in:fade>{ user.username }</h3>
                                    <p in:fade><b>id: </b>{user.id}</p>
                                    <p in:fade><b>Email: </b>{user.email}</p>
                                    <p in:fade><em><b>Created at</b> { user.created_at }</em></p>
                                </Box>
                            </li>
                        {:else}
                            <li>
                                <Box>
                                    <h3>No users found</h3>
                                    <p><em>Try creating some</em></p>
                                </Box>
                            </li>
                        {/each}
                    </ul>
                </div>
            {:catch}
                <p>ERROR!</p>
                <p>{ userList }</p>
            {/await}
        {/if}
</div>
