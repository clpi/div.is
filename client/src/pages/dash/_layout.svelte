<script>
  import { fade } from 'svelte/transition';
  /*import { isLogged, userData } from '../../store.js';*/
  import Sidenav from '../../comp/ui/sidenav.svelte';
  import ContentBg from '../../comp/ui/contentBg.svelte';
</script>
<style>
    .content {
        position: relative;
        margin-left: 5vw;
    }
    .db {
      min-height: 100vh;
    }
</style>
<div class="db">
  <ContentBg>
    <Sidenav/>
    <div class="content">
        <slot/>
    </div>
  </ContentBg>
</div>
