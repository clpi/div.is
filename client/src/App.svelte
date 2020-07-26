<script>
  import { Router } from "@sveltech/routify";
  import { routes } from "@sveltech/routify/tmp/routes";
  // TODO: Modularize reusable components (cards, ui elements, etc.)
  // TODO: Design your own buttons, etc. instead of bulma stuff
  // TODO: Make properties shared among comps inherited (ie fade in)
  </script>

<style>
    .body {
      height: 100%;
    }
    .contain {
        background-color: #fffefd;
    }

</style>
<div class="body">
    <Router {routes} />
    <div class="contain">
        <slot/>
    </div>
</div>
