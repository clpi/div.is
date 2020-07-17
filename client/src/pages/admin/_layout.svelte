<script>
  import { isActive, url, layout } from "@sveltech/routify";
  import { setContext, getContext, onMount } from 'svelte'
  import { fade } from 'svelte/transition';
  import Sidenav from '../../comp/ui/adm-nav.svelte'
</script>
<style>
    .content {
        position: relative;
        margin-left: 15vw;
    }
</style>
<div class="admin-db">
    <Sidenav/>
    <div class="content">
        <slot/>
    </div>
</div>
