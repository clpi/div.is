<!-- _layout.svelte -->
<script>
    import Nav from '../comp/ui/nav.svelte';
    import { afterPageLoad, url } from "@sveltech/routify";
    import { user, logged } from '../store.js';
    let isLoggedIn = false;
    let loggedIn = Promise.resolve([]);
    let userData = Promise.resolve([]);
    // TODO have everything load at once in fetch call
    // and then declare $ready instead of having await in DOM
    $afterPageLoad(async () => {
        handleRefresh();
        if (loggedIn.sub != null) {
            isLoggedIn = true;
        }
    })
    function handleRefresh() {
        loggedIn = getLogged();
    } 
    function handleUserData(id) {
        userData = getUser(id);
    }
    let logout = async () => {
        await fetch('http://localhost:3001/api/logout', {
            method: 'POST',
            credentials: 'include',
            headers: {
                cookie: document.cookie,
            }
        });
    }
    // TODO just return necessary user data in sub of jwt
    // TODO don't use JWT in cookie ... or do use JWT but in header?
    let getUser = async (id) => {
        const res = await fetch('http://localhost:3001/api/user/id/'+id)
            .then(res => res.json())
            .then(res => userData = res)
        return res;
    }
    let getLogged = async () => {
        const res = await fetch('http://localhost:3001/api/userstatus', {
          method: 'GET',
          credentials: 'include',
          headers: {
            cookie: document.cookie,
          }
        })
            /*.then(res => {*/
                /*if (res.headers.get("Authorized") == "false"){*/
                    /*isLoggedIn = false;*/
                    /*return res;*/
                /*} else {*/
                    /*isLoggedIn = true;*/
                /*}*/
                    
            /*})*/
            .then(res => res.json())
            .then(res => loggedIn = res);
        return res;
    }

</script>

<style>

</style>

<Nav/>
<button on:click={logout}>logout</button>
{#await loggedIn}
    <p> checking login status...</p>
{:then res}
    <p> {loggedIn.sub} </p>
    {#if loggedIn.sub != null}
        {#await getUser(res.sub)}
            <p> getting user data... </p>
        {:then userRes}
            <a href={$url('/user/:username', {username: userRes.username})}>{userRes.username}</a>
            <p> {userRes.email} </p>
            <p> {userData.email} {loggedIn.sub} {isLoggedIn}</p>
        {/await}
    {:else}
        <p> Not logged in </p>
    {/if}
{/await}
<slot />
