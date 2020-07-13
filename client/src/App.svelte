<script>
  import { Router } from "@sveltech/routify";
  import { routes } from "@sveltech/routify/tmp/routes";
  import { slide } from 'svelte/transition'
  import 'bulma/css/bulma.css'
  import { setContext, getContext, onMount } from 'svelte'
  onMount(async () => { 
      await fetch('http://localhost:3001/api/userstatus', {
        method: "GET",
        crdentials: "include",
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json',
            'Cache': 'no-cache',
            'Token': getContext("token"),
        }
      })
          .then(res => setContext("userData", res.json()))
          .then(res => setContext("loggedIn", res.ok))
          .catch(err => {
            console.log(err);
            setContext("userData", null);
            setContext("loggedIn", false);
              setContext("token", null);
          });
  });
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
