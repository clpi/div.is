<!-- _layout.svelte -->
<script>
    /*import Nav from '../comp/ui/nav.svelte';*/
    import { beforeUrlChange, url, isActive, ready } from "@sveltech/routify";
    import { onMount } from 'svelte';
    import { user, isLogged, logged } from '../store.js';
    import { slide, fade } from 'svelte/transition';
    // TODO have everything load at once in fetch call
    // and then declare $ready instead of having await in DOM
    // TODO handle auth like routify suggests on their website

    onMount(async () => {
      refresh();
    })
    $beforeUrlChange(async () => {
      refresh();
      return true;
    });
    let refresh = async () => {
      logged.set(await getLogged());
      user.set(await getUser());
      if ($logged != undefined) {
        isLogged.set(true);
      }
      console.log($logged);
      console.log($user);
      console.log($isLogged);
    }

    let logout = async () => {
      let res = await fetch(API_URL+'/auth/logout', {
          method: 'POST',
          credentials: 'include',
          headers: {
              cookie: document.cookie,
          }
      })
      .then(isLogged.set(false))
      .then(user.set({}))
      .then(logged.set({}))
      .catch(err => {
        console.error("Error:", err);
      });
        
      return res;
    }
    // TODO just return necessary user data in sub of jwt
    // TODO don't use JWT in cookie ... or do use JWT but in header?
    let getUser = async () => {
      const res = await fetch(API_URL+'/user/id/'+$logged.sub)
        .then(res => res.json())
        .catch(err => {
          console.error('Error:', err);
        });
      return res;
    }
    let getLogged = async () => {
      const res = await fetch(API_URL+'/auth/refresh', {
        method: 'GET',
        credentials: 'include',
        headers: {
          cookie: document.cookie,
        }
      })
        .then(res => res.json())
        .catch(err => {
          console.error('Error:', err)
        });
      return res;
    }
    getLogged().then($ready);
</script>

<style>
    .Navbar {
      background-color: rgba(0,0,0,0);
      margin-bottom: 0.3rem;
      position: sticky;
    }
    #navHome {
        margin-left: 2vw;
        margin-right: 5vw;
        border: 4px solid #fedaa1;
        padding: 0px 2px 0px 2px;
        border-radius: 2px;
        background-color: #FFF;
        box-shadow: 2px 2px 2px rgba(0,0,0,0.1);

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
        padding-left: 13.5vw;
        padding-right: 13.5vw;
        min-height: 100%;
    }
    .username {
    }
    .title {
      font-size: larger;
      font-weight: 400;
    }
    .user-corner {
      background-color: #fff;
      /*border: 2px solid #eeee;*/
    }
    .wrapper {
      height: 100%;
    }
</style>

<div class="wrapper">
    <div class="Navbar">
      <ul in:fade={{delay:0, duration: 1000}} class="Nav">
            <li id="navHome"class:active={$isActive("/")}>
                <a class="title" href={$url("/")}>div.is</a>
            </li>
            <li class:active={$isActive("/index")}>
                <a href={$url("/index")}>home</a>
            </li>
            <li class:active={$isActive("/about")}>
                <a href={$url("/about")}>about</a>
            </li>
            <li class:active={$isActive("/contact")}>
                <a href={$url("/contact")}>contact</a>
            </li>
            {#await refresh}
            {:then}
            {#if $isLogged}
              <li class:active={$isActive("/dash")}>
                  <a href={$url("/dash")}>dash</a>
              </li>
              <li class:active={$isActive("/admin")}>
                  <a href={$url("/admin")}>admin</a>
              </li>
              <div class="user-corner">
                <li 
                    in:fade
                    id="loginNav" 
                >
                <a on:click={logout} href="/">log out</a>
                </li>
                <li 
                    in:fade 
                    id="loginNav" 
                    class:active={$isActive("/"+$user.username)}
                >
                  <a class="username"
                  href={$url('/:username', {username: $user.username})}>
                        {$user.username} ⇩
                  </a>
                </li>
              </div>
              {:else}
              <div class="corner">
                <li id="signupNav" class:active={$isActive("/signup")} >
                    <a href={$url("/signup")}>signup</a>
                </li>
                <li id="loginNav" class:active={$isActive("/login")}>
                    <a href={$url("/login")}>login</a>
                </li>
              </div>
            {/if}
            {:catch}
                <li id="signupNav" class:active={$isActive("/signup")} >
                    <a href={$url("/signup")}>signup</a>
                </li>
                <li id="loginNav" class:active={$isActive("/login")}>
                    <a href={$url("/login")}>login</a>
                </li>
            {/await}
        </ul>
    </div>
    <div class = "content">
      <slot scoped={{usr: $user, log: $isLogged}}/>
      <!-- 
        <slot scoped={{user: userData, loggedIn: loggedIn}}/> 
      -->
    </div>
</div>
