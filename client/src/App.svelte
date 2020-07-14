<script>
  import { Router } from "@sveltech/routify";
  import { routes } from "@sveltech/routify/tmp/routes";
  import { slide } from 'svelte/transition'
  import 'bulma/css/bulma.css'
  import { setContext, getContext, onMount } from 'svelte'
  let loggedIn = getContext('loggedIn'); 
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
