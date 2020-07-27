<script>
  // TODO make general sidenav component that is passed
  //      an array of links which it renders through a for loop
  import { isActive, url, layout } from "@sveltech/routify";
  import { fade } from 'svelte/transition';
</script>
<style>
    .admin-nb {
        float: left;
        min-width: auto;
        margin-right: 10%;
        height: 100%;
        /*margin-right: 5vw;*/
        /*position: fixed;*/
    }
    .admin-nav {
        margin-left: auto;
        max-width: 1200px;
        align-items: center;
        align-content: center;
    } 
    ul {
        /*margin-left: 5vw;*/
        list-style-type: none;
        justify-items: auto;
        margin-top: 1rem;
        margin-bottom: 1.5rem;
        padding: 0;
        overflow: hidden;
    }
    li {
      min-width:100%;
      /*margin: auto;*/
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
      font-size: 1.1rem;
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
        border-top: 1px solid #eee;
        border-bottom: 1px solid #eee;
        border-right: 1px solid #eee;
        box-shadow: 1px 1px 1px rgba(0,0,0,0.1);
    }
    .active:hover {
        border-left-color: #fb5;
        background-color: #fefafa;
    }
    .nav-head {
        margin-bottom: 20px; 
    }
    .nav-ht {
        font-size: 1.6rem;
        margin-left: 20px;
        font-weight: 400;
    }
</style>
<div class="admin-nb">
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
        <li in:fade class:active={$isActive("/admin/stats")}>
            <a href={$url("/admin/stats")}>stats</a>
        </li>
    </ul>
</div>
