<script>
  import {Collapse, Field, Button, Input, Icon, Tabs, Tab } from 'svelma'
</script>

<style>
    .dbcontainer{
        margin: auto;
        margin-top: 2rem;
        display: inline;
    }
    .title {
        font-size: 2em;
        text-shadow: 0px 2px 2px #555;
        font-weight:400;
        
    }
    .subtitle {
        font-size:1.2em;
        text-shadow: 0px 1.5px 1.5px #555;
        font-weight: 500;
    }
    .hero{
        background-image: linear-gradient(170deg, #ef8958, #ffdeaa);
        height: 10rem;
        color: #221;
        border-bottom: 4px solid rgba(255, 140, 100, 40)
    }
    .buttons{
        padding: 20px 10px 20px 10px;
    }
    .searchbar {
        width:max-content;
    }
</style>
<section class="hero is-success">
    <div class="hero-body">
      <div class="container">
        <h1 class="title">
          about
        </h1>
        <h2 class="subtitle">
          Logging website
        </h2>
      </div>
    </div>
  </section>
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
