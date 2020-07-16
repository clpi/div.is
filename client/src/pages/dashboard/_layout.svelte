<script>
  import { isActive, url, layout } from "@sveltech/routify";
  import { fade } from 'svelte/transition';
  import { setContext, getContext, onMount } from 'svelte'
  let userData = getContext("userData");
  let loggedIn = getContext("loggedIn");
</script>
<style>
    .db {
        margin-left: 15%;
        margin-right: 10%;
    }
    .Navbar {
        float: left;
        min-width: auto;
        margin-right: 7vw;
    }
    .NavLinks {
        margin-bottom: 1rem;
    }
    .Nav {
        margin-left: auto;
        margin-right: auto;
        max-width: 1200px;
        align-items: center;
        align-content: center;
    } 
    ul {
        margin-left: 5vw;
        list-style-type: none;
        justify-items: auto;
        margin-top: 1rem;
        margin-bottom: 1.5rem;
        padding: 0;
        overflow: hidden;
    }
    li {
        min-width:auto;
        margin: auto;
        align-items: center;
        vertical-align: center;
        border-left: 4px solid rgba(255,170,80,0);
    }
    li a {
        display: inline-block;
        color: #1f1f1f;
        text-align: center;
        padding: 1.8vh 1.5vw 1.8vh 1.5vw;
        text-decoration: none;
        font-stretch: expanded;
        font-weight:400;
        outline:0;
    }
    li:hover {
        border-left: 4px solid #ffdca9;
        transition: border-left-color 0.2s;
    }
    .active {
        border-left-color: #fa4;
        align-items: center;
    }
    .active:hover {
        border-left-color: #fa4;
    }
    .nav-head {
        margin-bottom: 20px; 
    }
    .nav-ht {
        font-size: 1.4rem;
    }

</style>
<div class="db">
    <div class="Navbar">
        <div class="NavLinks">
            <ul class="Nav">
                <li in:fade class="nav-head">
                    <p class="nav-ht">dashboard</p>
                </li>
                <li in:fade class:active={$isActive("/dashboard/index")}>
                    <a href={$url("/dashboard/index")}>home</a>
                </li>
                <li in:fade class:active={$isActive("/dashboard/records")}>
                    <a href={$url("/dashboard/records")}>records</a>
                </li>
                <li in:fade class:active={$isActive("/dashboard/items")}>
                    <a href={$url("/dashboard/items")}>items</a>
                </li>
                <li in:fade class:active={$isActive("/dashboard/entries")}>
                    <a href={$url("/dashboard/entries")}>entries</a>
                </li>
                <li in:fade class:active={$isActive("/dashboard/stats")}>
                    <a href={$url("/dashboard/stats")}>stats</a>
                </li>
            </ul>
        </div>
    </div>
    <div class="content">
        <slot/>
    </div>
</div>
