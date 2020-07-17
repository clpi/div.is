<!-- _layout.svelte -->
<script>
    /*import Nav from '../comp/ui/nav.svelte';*/
    import { afterPageLoad, url, isActive } from "@sveltech/routify";
    import { user, logged } from '../store.js';
    import { slide, fade } from 'svelte/transition';
    let isLoggedIn = false;
    let loggedIn = Promise.resolve([]);
    let userData = Promise.resolve([]);
    // TODO have everything load at once in fetch call
    // and then declare $ready instead of having await in DOM
    $afterPageLoad(async () => {
        handleRefresh();
        if (loggedIn.sub != null) {
            isLoggedIn = true;
        }
    })
    function handleRefresh() {
        loggedIn = getLogged();
    } 
    function handleUserData(id) {
        userData = getUser(id);
    }
    let logout = async () => {
        await fetch('http://localhost:3001/api/logout', {
            method: 'POST',
            credentials: 'include',
            headers: {
                cookie: document.cookie,
            }
        });
    }
    // TODO just return necessary user data in sub of jwt
    // TODO don't use JWT in cookie ... or do use JWT but in header?
    let getUser = async (id) => {
        const res = await fetch('http://localhost:3001/api/user/id/'+id)
            .then(res => res.json())
            .then(res => userData = res)
        return res;
    }
    let getLogged = async () => {
        const res = await fetch('http://localhost:3001/api/userstatus', {
          method: 'GET',
          credentials: 'include',
          headers: {
            cookie: document.cookie,
          }
        })
            /*.then(res => {*/
                /*if (res.headers.get("Authorized") == "false"){*/
                    /*isLoggedIn = false;*/
                    /*return res;*/
                /*} else {*/
                    /*isLoggedIn = true;*/
                /*}*/
                    
            /*})*/
            .then(res => res.json())
            .then(res => loggedIn = res);
        return res;
    }

</script>

<style>
    .Navbar {
        /*border-top: 4px solid #feba80;*/
        background-image: linear-gradient(#fffffd, #fffdfe);
        /*margin-top: 0.5rem;*/
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
        margin-top:1rem;
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
    .content {
        padding-left: 15.5vw;
        padding-right: 15.5vw;
    }
</style>

<div class="wrapper">
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
            {#await loggedIn}
            {:then res}
            {#if loggedIn.sub != null}
            {#await getUser(res.sub)}
            {:then userRes}
                <li 
                    in:fade
                    id="loginNav" 
                >
                <a on:click={logout} href="/">log out</a>
                </li>
                <li 
                    in:fade 
                    id="loginNav" 
                    class:active={$isActive("/"+userRes.username)}
                >
                    <a href={$url('/:username', {username: userRes.username})}>
                        {userRes.username}
                    </a>
                </li>
            {/await}
            {/if}
            {:catch}
                <li in:fade id="signupNav" class:active={$isActive("/signup")} >
                    <a href={$url("/signup")}>signup</a>
                </li>
                <li in:fade id="loginNav" class:active={$isActive("/login")}>
                    <a href={$url("/login")}>login</a>
                </li>
            {/await}
            <li in:slide={{delay:300}} id="navIcon">
                <a href="http://github.com/pecusys">
                    gh
                </a>
            </li>
        </ul>
    </div>
    </div>
    <div class = "content">
        <slot />
    </div>
</div>
