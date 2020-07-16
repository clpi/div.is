<!-- _layout.svelte -->
<script>
    import Nav from '../comp/ui/nav.svelte';
    import { afterPageLoad } from "@sveltech/routify"
    let loggedIn = false;
    $afterPageLoad(async () => {
        loginStatus();
        setContext("loggedIn", loggedIn);
        setContext("userData", userData)
    })
    let checkLogged = async () => {
        const res = await fetch('http://localhost:3001/api/userstatus', {
          method: 'GET',
          credentials: 'include',
          headers: {
            cookie: document.cookie,
          }
        });
        if (!res.ok) {
          setContext('loggedIn', false);
          setContext('userData', null);
          loggedIn = false;
          return false;
        } else {
          loggedIn = true;
            return true;
        }
    }
    let loginStatus = () => {
        promise = checkLogged();
    }

</script>

<style>

</style>

<Nav/>
{#await loggedIn}
    <p>Checking login status...</p>
{:then logged}
        {logged}
        {loggedIn}
{:catch}
    <p>Couldnt fetch login status</p>
{/await}

<slot />
