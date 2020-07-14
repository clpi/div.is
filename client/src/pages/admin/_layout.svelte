<script>
  import { isActive, url, layout } from "@sveltech/routify";
  import { Button, Collapse, Icon, Field, Input, Modal} from 'svelma';
  import { setContext, getContext, onMount } from 'svelte'
  import { fade } from 'svelte/transition';
</script>
<style>
    .admin-db {
        margin-left: 15%;
        margin-right: 10%;
    }
    .admin-nb {
        float: left;
        min-width: auto;
        margin-right: 7vw;
    }
    .admin-nbl {
        margin-bottom: 1rem;
    }
    .admin-nav {
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
    .wrapper {
        border-right: 2px solid #fa4;
    }

</style>
<div class="admin-db">
    <div class="admin-nb">
        <div class="admin-nbl">
            <ul class="admin-nav">
                <li in:fade class="nav-head">
                    <p class="nav-ht">admin center</p>
                </li>
                <li in:fade class:active={$isActive("/admin/index")}>
                    <a href={$url("/admin/index")}>home</a>
                </li>
                <li in:fade class:active={$isActive("/admin/db")}>
                    <a href={$url("/admin/db")}>database</a>
                </li>
                <li in:fade class:active={$isActive("/admin/users")}>
                    <a href={$url("/admin/users")}>users</a>
                </li>
                <li in:fade class:active={$isActive("/admin/records")}>
                    <a href={$url("/admin/records")}>records</a>
                </li>
                <li in:fade class:active={$isActive("/dashboard/stats")}>
                    <a href={$url("/admin/stats")}>stats</a>
                </li>
            </ul>
        </div>
    </div>
    <div class="content">
        <div class="wrapper">
            <slot/>
        </div>
    </div>
</div>
