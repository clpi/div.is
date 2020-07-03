<script>
  import { Router } from "@sveltech/routify";
  import { routes } from "@sveltech/routify/tmp/routes";
  import { slide } from 'svelte/transition'
  import 'bulma/css/bulma.css'
  </script>

<style>
    body {
        min-width: 70vw;
        max-height: 100vh;
        padding-left:14em;
    }
    .contain {
        min-width: max-content;
        margin: auto;
        background-color: #fffefd;
        min-height:100vh;
    }

</style>
<div class="contain">
    <slot/>
<Router {routes} />
</div>
