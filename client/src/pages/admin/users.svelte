<script>
  import { Tabs, Tab, Button, Field, Input } from 'svelma'
  import { slide, fade } from 'svelte/transition'
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
    .box {
		width: 300px;
        float: left;
		border: 1px solid #fa4;
		border-radius: 2px;
		box-shadow: 2px 2px 8px rgba(0,0,0,0.1);
		padding: 1em;
        margin: 1em;
	}
    h1 {
        font-size: 2rem;
        font-weight: 300;
    }
    h2 {
        font-size: 1.6rem;
        font-weight: 300;
    }
    h3 {
        font-size: 1.3rem;
        font-weight: 300;
    }
    ul {
        list-style-type: none;
        display: inline-block;
    }
    p { font-size: 1em; }
    li {
        float: left;
    }
    .utab {
        padding-left: 10px;
    }
</style>

<div in:fade={{duration:100}}>
    <h1>User Admininistration</h1>
    <Tabs>
        <Tab class="utab" label = "Get">
            <h2>Get user<h2>
            <ul>
                <li>
                    <div class="box">
                        <Field label="By username"><Input label="Username" bind:value={userUsername}></Input></Field>             
                        <Button on:click={handleUsername}>Submit</Button>

                            <Field label="By email"><Input label="Username" bind:value={userEmail}></Input></Field>             
                        <Button on:click={handleEmail}>Submit</Button>
                            <Field label="By id"><Input label="Username" bind:value={userId}></Input></Field>             
                        <Button on:click={handleId}>Submit</Button>
                    </div>
                </li>
                {#if submittedUser}
                    {#await userPromise}
                        <p>Getting user...</p>
                    {:then user}
                        <li>
                            <div class="box" in:fade>
                                <p in:fade>{ user.username }</p>
                                <p in:fade><b>id: </b>{user.id}</p>
                                <p in:fade><b>Email: </b>{user.email}</p>
                                <p in:fade><em><b>Created at</b> { user.created_at }</em></p>
                                
                            </div>
                        </li>
                    {:catch}
                        <p>No user found.</p>
                    {/await}

                {/if}
            </ul>
        </Tab>
        <Tab class="utab" label = "Create">
            <h2>Create users/<h2>
        </Tab>
    <Tab label="Users" class="utab">
        <h2>Fetch users</h2>
        <div class="users-tab">
            <div>
                <Button 
                    type="is-primary" 
                    nativeType="submit" 
                    on:click={ fetchUsers }
                >
                    Fetch
                </Button>
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
                                    <div class="box" in:slide>
                                        <h3 in:fade>{ user.username }</h3>
                                        <p in:fade><b>id: </b>{user.id}</p>
                                        <p in:fade><b>Email: </b>{user.email}</p>
                                        <p in:fade><em><b>Created at</b> { user.created_at }</em></p>
                                    </div>
                                </li>
                            {:else}
                                <li>
                                    <div class="box">
                                        <h3>No users found</h3>
                                        <p><em>Try creating some</em></p>
                                    </div>
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
    </Tab>
    </Tabs>
</div>
