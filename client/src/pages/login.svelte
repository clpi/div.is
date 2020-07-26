<script>
  import { slide, fade }from 'svelte/transition'
  import { user, isLogged, logged, session } from '../store.js';
  import { goto } from '@sveltech/routify';
  import { metatags } from '@sveltech/routify';
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
      padding-left: 5vw;
    }
    .loginForm {
      margin-top: 40px;
      background-color: #fefdfd;
      border: 2px solid #eee;
      border-bottom-color: #fd9;
      border-bottom-width: 3px;
      border-radius: 8px;
      padding-top: 5%;
      padding-bottom: 10%;
      align-items: center;
      padding-left: 15.5vw;
      padding-right: 15.5vw;
      box-shadow: 1px 2px 2px rgba(0,0,0,0.1);
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
    }
    .loading {
        box-shadow: 2px 2px #000;
    }
    .buttons {
      padding-top: 40px;
    }
</style>

<div class="login">
    <div in:fade={{duration:100}} class="loginForm">
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
    </div>
</div>
