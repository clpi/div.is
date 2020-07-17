<script>
  import { isActive, url, layout } from "@sveltech/routify";
  import { setContext, getContext, onMount } from 'svelte'
  import { fade } from 'svelte/transition';
  import Sidenav from '../../comp/ui/adm-nav.svelte'
</script>
<style>
    .admin-db {
        margin-left: 15%;
        margin-right: 10%;
    }
    .content {
        padding-top: 5%;
    }
</style>
<div class="admin-db">
    <Sidenav/>
    <div class="content">
        <slot/>
    </div>
</div>
