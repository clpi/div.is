<script>
  import { isActive, url, layout } from "@sveltech/routify";
  import { setContext, getContext, onMount } from 'svelte'
  import { fade } from 'svelte/transition';
  import Sidenav from '../../comp/ui/adm-nav.svelte'
  import ContentBg from '../../comp/ui/contentBg.svelte'
</script>
<style>
    .content {
        position: relative;
        margin-left: 5vw;
    }
    .admin-db {
      min-height: 100vh;
    }
</style>
<div class="admin-db">
    <ContentBg>
    <Sidenav/>
    <div class="content">
        <slot/>
    </div>
    </ContentBg>
</div>
