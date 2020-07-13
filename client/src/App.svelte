<script>
  import { Router } from "@sveltech/routify";
  import { routes } from "@sveltech/routify/tmp/routes";
  import { slide } from 'svelte/transition'
  import 'bulma/css/bulma.css'
  import { setContext, getContext, onMount } from 'svelte'
  import { session } from 'login.svelte'
      /*await fetch('http://localhost:3001/api/userstatus', {*/
        /*method: "GET",*/
        /*crdentials: "include",*/
        /*headers: {*/
            /*'Accept': 'application/json',*/
            /*'Content-Type': 'application/json',*/
            /*'Cache': 'no-cache',*/
            /*'Token': getContext("token"),*/
        /*}*/
      /*})*/
          /*.then(res => setContext("userData", res.json()))*/
          /*.then(res => setContext("loggedIn", res.ok))*/
          /*.catch(err => {*/
            /*console.log(err);*/
            /*setContext("userData", null);*/
            /*setContext("loggedIn", false);*/
              /*setContext("token", null);*/
          /*});*/
  /*});*/
  let loggedIn = getContext('loggedIn'); 
  let userData = getContext('userData');
  onMount(async () => {
      if (loggedIn == undefined || userData == undefined) {
        setContext('loggedIn', false);
        setContext('userData', null);

      }
      if (loggedIn) {
          let res = await fetch('http://localhost:3001/api/userstatus', {
              method: 'GET',
              credentials: 'include',
              headers: {
                cookie: document.cookie,
              }
          });
          if (!res.ok) {
            setContext('loggedIn', false);
            setContext('userData', null);
          }
      }
      if (logged == undefined) {
          logged = writable(false);
          session = wriable(null);
          duration = writable(0);
      }
    userData = getContext('userData');
  })
  // TODO: Modularize reusable components (cards, ui elements, etc.)
  // TODO: Design your own buttons, etc. instead of bulma stuff
  // TODO: Make properties shared among comps inherited (ie fade in)
  </script>

<style>
    .body {
        height: 100%;
    }
    .contain {
        min-width: max-content;
        margin: auto;
        background-color: #fffefd;
    }

</style>
<div class="body">
    <Router {routes} />
    <div class="contain">
        <slot/>
    </div>
</div>
