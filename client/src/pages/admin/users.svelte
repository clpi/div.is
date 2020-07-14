<script>
  import { Tabs, Tab, Button, Field, Input } from 'svelma'
  import { slide, fade } from 'svelte/transition'
  let userList;
  let promise = Promise.resolve([]);
  let submitted = false;
  async function fetchAll() {
      const usrs = await fetch('http://localhost:3001/api/user/all', {
          method: 'GET',
          headers: {
            'content-type': 'application/json',
              /*authorization: <authorization>*/
          },
      });
      if (usrs.ok) {
          userList = usrs;
          return usrs;
      } else {
        throw new Error(users);
      }
    }
  function fetchUsers() {
    promise = fetchAll();
    submitted = true;
  }

</script>
<style>
    .signupForm{
        padding-left: 15.5vw;
        padding-right: 15.5vw;
    }
    .box {
		width: 300px;
        float: left;
		border: 1px solid #fa4;
		border-radius: 2px;
		box-shadow: 2px 2px 8px rgba(0,0,0,0.1);
		padding: 1em;
		margin: 1em 1em 1em 1em;
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
</style>

<div in:fade={{duration:100}} class="signupForm">
    <h1>User Admininistration</h1>
    <Tabs>
        <Tab label = "Create">
            <h2>Create users/<h2>
        </Tab>
    <Tab label="Users">
        <h2>Fetch users</h2>
        <p>{ userList }</p>
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
                    <p>PFetching usres...</p>
                {:then users}
                    <p>{users}</p>
                    <div>
                        <ul>
                            {#each users.json() as user}
                                <li>
                                    <div class="box">
                                        <h3>{ user.usrename }</h3>
                                        <p><em>Created at { user.created_at }</em></p>
                                        <p><b>id: </b>{user.id}</p>
                                        <p><b>Email: </b>{user.email}</p>
                                        <p><b>Password: </b>{user.password}</p>
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
