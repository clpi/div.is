<script>
  import { isActive, url, layout } from "@sveltech/routify";
  import { Button, Collapse, Icon, Field, Input, Modal} from 'svelma';
  import { slide } from 'svelte/transition';
</script>
<style>
    .Navbar {
        align-items: center;
        align-content: center;
    }
    .NavLinks {
        margin-bottom: 1rem;
        align-items: center;
        align-content: center;
    }
    .Nav {
        margin-left: auto;
        margin-right: auto;
        max-width: 1200px;
        display: block;
        align-items: center;
        align-content: center;
    } 
    ul {
        list-style-type: none;
        justify-items: auto;
        align-items: center;
        align-content: center;
        margin-top: 1rem;
        margin-bottom: 1.5rem;
        padding: 0;
        overflow: hidden;
    }
    li {
        float: left;
        min-width:auto;
        margin: auto;
        align-items: center;
        vertical-align: center;
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
        border-bottom: 4px solid #ffdca9;
        transition: border-bottom-width 0.2s;
    }
    .active {
        border-bottom: 4px solid #fa4;
        align-items: center;
    }
    .active:hover {
        border-bottom: 4px solid #fa4;
    }

</style>
<div class="Navbar">
    <div class="NavLinks">
        <ul class="Nav">
            <li in:slide={{delay:150}} class:active={$isActive("/dashboard/index")}>
                <a href={$url("/dashboard/index")}>home</a>
            </li>
            <li in:slide={{delay:150}} class:active={$isActive("/dashboard/records")}>
                <a href={$url("/dashboard/records")}>records</a>
            </li>
            <li in:slide={{delay:200}} class:active={$isActive("/dashboard/items")}>
                <a href={$url("/dashboard/items")}>items</a>
            </li>
            <li in:slide={{delay:250}} class:active={$isActive("/dashboard/entries")}>
                <a href={$url("/dashboard/entries")}>entries</a>
            </li>
            <li in:slide={{delay:300}} class:active={$isActive("/dashboard/stats")}>
                <a href={$url("/dashboard/stats")}>stats</a>
            </li>
        </ul>
    </div>
</div>
