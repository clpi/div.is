<script>
  import { slide, fade }from 'svelte/transition'
  import { user, isLogged, logged, session } from '../store.js';
  import { goto } from '@sveltech/routify';
  import { metatags } from '@sveltech/routify';
  import ContentBg from '../comp/ui/contentBg.svelte';
  metatags.title = "login";

  let loginInfo = {
    username: '',
    password: '',
  }

  let toast = () => {
      console.log("toasted");
  }
  let fetching = false;
  let disabled = false;
  let promise = Promise.resolve([]);
  let submitted = false;
  async function loginUser(userInfo) {
      const loginPost = await fetch(API_URL+'/auth/login', {
          method: 'POST',
          credentials: 'include',
          headers: {
            'content-type': 'application/json',
              /*authorization: <authorization>*/
          },
          body: JSON.stringify(userInfo)
      });
      if (loginPost.ok) {
        isLogged.set(true);
        return loginPost.json();
      } else {
        throw new Error(users);
      }
    }
  function handleLogin() {
    fetching = true;
    disabled = true;
    submitted = true;
    promise = loginUser(loginInfo);
  }
  // allow username OR email for signin
</script>

<style>
    .login {
      padding-left: 10vw;
      padding-right: 10vw;
    }
    .login-left{
      padding-left: 10%;
      padding-bottom: 10%;
    }
    .login-head {
      display: inline-block;
      margin: auto;
      text-align: center;
    }
    .title {
      font-weight: 300;
      text-shadow: -0px -1px 1px #fff;
      font-size: 2.5rem;
    }
    h1 { 
        font-size: 2rem;
        font-weight: 300;
        text-shadow: 0px 1px 1px #444;
    }
    h2 {
        font-size: 1.6rem;
        font-weight: 300;
    }
    h3 {
        font-size: 1.3rem;
        font-weight: 300;
    } .loading {
        box-shadow: 2px 2px #000;
    }
</style>

<div class="login">
    <ContentBg>
      <div class="login-left">
      <div class="login-head">
        <h1 class="title" in:fade={{duration: 600}}>Welcome back! </h1>
      </div>
      <form label="login" in:fade={{duration: 600}}> 
            <h3>Username</h3>
            <input type="username" bind:value={loginInfo.username} />
            <h3>Password</h3>
            <input type="password" bind:value={loginInfo.password} passwordReveal={true} />
        </form>
        <div class="buttons, submitAuth">
            <button 
                type="submit" 
                class={fetching ? 'loading' : ''}
                on:click={ handleLogin } { disabled } { toast }
                >
                Login
            </button>
        </div>
        <div class=loginRes>
            {#await promise}
                <p>waiting...</p>
            {:then userData}
                {#if submitted}
                    <p>{userData}</p>
                    <p>Welcome {userData.username}!</p>
                    <p>Your ID: {userData.uid}</p>
                    <button on:click={()=>{$goto('/index')}}>Go home</button>
                {/if}
            {:catch error}
                <p style="color: red">{error.message}</p>
            {/await}
        </div>
      </div>
      <div class="login-right">
      </div>
    </ContentBg>
</div>
