<script>
  import {Collapse, Field, Button, Input, Icon, Tabs, Tab } from 'svelma'
  import Hero from '../../comp/hero.svelte'
  import { slide, fade } from 'svelte/transition'
  import { setContext, getContext, onMount } from 'svelte'
  onMount(async () => { 
      await fetch('http://localhost:3001/api/userstatus')
          .then(res => setContext("userData", res.json()))
          .then(res => setContext("loggedIn", res.ok))
          .catch(err => {
            console.log(err);
            setContext("userData", null);
            setContext("loggedIn", false);
          });
  });
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
    h1 {
        font-size: 2rem;
        font-weight: 300;
    }
</style>
<div class="dbindex" in:fade={{duration:100}}>
    <h1>dashboard</h1>
    <div class="buttons">
        <Button type="is-success" iconPack="fa" iconLeft="plus">Save</Button>
        <Button iconPack="fa" iconLeft="edit">Save</Button>
            <Button outline iconPack="fa" iconLeft="times">Delete</Button>
            <div class="searchbar">
                <Field id="searchbar">
                    <Input type="search" placeholder="Search" icon="search" position="is-centered"/>
                    <p class="control">
                        <Button type="is-primary">Search</Button>
                    </p>
                </Field>
            </div>
    </div>

    <div class=dbcontainer>
        <Tabs style="is-fullwidth" position="is_centered">
        <Tab label="Home" icon="users">
            <Button iconPack="fab" iconLeft="github">Github</Button>
        </Tab>
        <Tab label="Records" icon="map-marker-alt">
            <div class="recordOptions">
                <Button iconPack="fab" iconLeft="add" type= "is-primary">Update</Button>
                    <Button iconPack="fab" iconLeft="remove" type="is-danger">No Data</Button>
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
        <Tab label="Items" icon="ellipsis-h"></Tab>
            <Tab label="Entries" icon="ellipsis-h"></Tab>
        </Tabs>
    </div>
</div>
