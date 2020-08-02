<script lang="ts">
  import * as dialog from 'tauri/api/dialog';
  /*import { emit, listen } from 'tauri/api/event';*/
  import { setTitle } from 'tauri/api/window';
  import { getAllUsers } from '../util/calls';
  import Button from '../comp/ui/button.svelte';
  /*import {  } from 'svelte-simple-forms-lib';*/
  function open(): void {
    dialog.open();
  };

  function save(): void {
    dialog.save();
  };

  function login(): void {
    setTitle("Login to div.is");
  }
  function test(): void {
    setTitle("Login to div.is");
  }
  let userPromise = Promise.resolve([]);
  let submittedUsers: boolean = false;
  async function getUsers(): Promise<JSON> {
    submittedUsers = true;
    setTitle("Fetching users...");
    const res = await getAllUsers()
      .catch(err=>{
      console.log(err);
        return [];
    });
    userPromise = res;
    setTitle("Users fetched!");
    return res;
  }
</script>

<style>
</style>

<div>
  <h3>Home</h3>
  <Button on:click={open} label="Open"/>
  <Button on:click={test} label="test"/>
    <Button on:click={save} label="Save"/>
    <Button on:click={login} label="Login"/>
    <Button on:click={getUsers} label="Get users"/>

  {#await userPromise}
    <p>Getting users...</p>
  {:then users}
    {#if submittedUsers}
      <p>{ users }</p>
      {#each users as user}
        <ul>
          <h3>User</h3>
          <li><p>{user.id}</p></li>
          <li><p>{user.email}</p></li>
          <li><p>{user.username}</p></li>
          <li><p>{user.password}</p></li>
          <li><p>{user.createdAt}</p></li>
        </ul>
      {/each}

    {/if}

  {:catch err}
    <p>{ err } - Something happened. Couldn't get users</p>
  {/await}
</div>
