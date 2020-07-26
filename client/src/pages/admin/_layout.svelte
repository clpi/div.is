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
</style>
<div class="admin-db">
    <Sidenav/>
    <div class="content">
      <ContentBg>
        <slot/>
      </ContentBg>
    </div>
</div>
