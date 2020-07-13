<!-- _layout.svelte -->
<script>
  import { isActive, url, layout } from "@sveltech/routify";
  import { Button, Collapse, Icon, Field, Input, Modal} from 'svelma';
  import { slide } from 'svelte/transition';
  import { setContext, getContext, onMount } from 'svelte'
  let loggedIn=getContext("loggedIn");
  let userData=getContext("userData");
  function logout() {
  } 

</script>

<style>
    .Navbar {
        border-top: 4px solid #feba80;
        background-image: linear-gradient(#fffffd, #fffdfe);
    }
    .NavLinks {
        margin-bottom: 1rem;
    }
    #navHome {
        margin-left: 5vw;
        margin-right: 5vw;
        border: 4px solid #fedaa1;
        padding: 0px 4px 0px 4px;

    }
    #navHome:hover {
        color: #fe8a00;
        background-color: #feeada;
    }
    .Nav {
        margin-left: auto;
        margin-right: auto;
        max-width: 1200px;
        display: block;
    } 
    ul {
        list-style-type: none;
        justify-items: auto;
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
    #loginNav {
        float: right;
        color: pink !important;
    }
    #signupNav {
        color: red;
        float: right;
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
    #navIcon{
        float: right;
    }

</style>
<div class="Navbar">
    <div class="NavLinks">
        <ul class="Nav">
            <li in:slide={{delay:100}} id="navHome"class:active={$isActive("/")}>
                <a href={$url("/")}>memri</a>
            </li>
            <li in:slide={{delay:150}} class:active={$isActive("/index")}>
                <a href={$url("/index")}>home</a>
            </li>
            <li in:slide={{delay:200}} class:active={$isActive("/dashboard")}>
                <a href={$url("/dashboard")}>dashboard</a>
            </li>
            <li in:slide={{delay:250}} class:active={$isActive("/about")}>
                <a href={$url("/about")}>about</a>
            </li>
            <li in:slide={{delay:300}} class:active={$isActive("/contact")}>
                <a href={$url("/contact")}>contact</a>
            </li>
            <li in:slide={{delay:350}} class:active={$isActive("/admin")}>
                <a href={$url("/admin")}>admin</a>
            </li>
            {#if !loggedIn}
            <li in:slide={{delay:200}} id="signupNav" class:active={$isActive("/signup")} >
                <a href={$url("/signup")}>signup</a>
            </li>
            <li in:slide={{delay:250}} id="loginNav" class:active={$isActive("/login")}>
                <a href={$url("/login")}>login</a>
            </li>
            {:else}
                <li 
                    in:slide={{delay:250}} 
                    id="loginNav" 
                    class:active={$isActive("/u/"+userData.username)}
                >
                <a on:click={logout} href="/">log out</a>
                </li>
                <li 
                    in:slide={{delay:250}} 
                    id="loginNav" 
                    class:active={$isActive("/u/"+userData.username)}
                >
                <a href={$url("/u/"+userData.username)}>{ userData.username }</a>
                </li>
            {/if}
            <li in:slide={{delay:300}} id="navIcon">
                <a href="http://github.com/pecusys">
                    <Icon pack="fab" size="is-small" icon="github"/>
                </a>
            </li>
        </ul>
    </div>
</div>
<slot />
