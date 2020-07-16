<script>
  import { slide, fade } from 'svelte/transition'
  import { setContext, getContext, onMount } from 'svelte'
  import Box from '../../comp/ui/box.svelte';
  import Tab from '../../comp/ui/tab.svelte';
  let userData = getContext("userData");
  let loggedIn = getContext("loggedIn");
  console.log(userData);
  console.log(loggedIn);
</script>

<style>
    .dbcontainer{
    }
    .buttons{
        padding: 20px 10px 20px 10px;
    }
    .searchbar {
        width:max-content;
    }
    .dbindex {
    }
</style>
<div class="dbindex" in:fade={{duration:100}}>
    <h1>dashboard</h1>
    <div class="buttons">
        <button>Save</button>
        <button>Save</button>
            <button>Delete</button>
            <div class="searchbar">
                <form id="searchbar">
                    <input type="search" placeholder="Search"/>
                    <p class="control">
                        <button>Search</button>
                    </p>
                </form>
            </div>
    </div>

    <div class=dbcontainer>
            <Tab label="Home">
            <button>Github</button>
        </Tab>
        <Tab label="Records">
            <div class="recordOptions">
                <button>Update</button>
                    <button>No Data</button>
            </div>
            <table class="table is-fullwidth">
                <thead>
                    <tr>
                        <th></th>
                        <th>Name</th>
                        <th>Items</th>
                        <th>Last entry</th>
                        <th>Tags</th>
                    </tr>
                </thead>
                <tbody>
                </tbody>
            </table>
        </Tab>
        <Tab label="Items"></Tab>
            <Tab label="Entries"></Tab>
    </div>
</div>
