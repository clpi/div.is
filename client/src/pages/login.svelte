<script>
  import { slide, fade }from 'svelte/transition'
  import { setContext, getContext, onMount } from 'svelte'
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
      const loginPost = await fetch(API_URL+'/login', {
          method: 'POST',
          credentials: 'include',
          headers: {
            'content-type': 'application/json',
              /*authorization: <authorization>*/
          },
          body: JSON.stringify(userInfo)
      });
      if (loginPost.ok) {
        setContext("loggedIn", true);
        setContext("userData", loginPost);
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
    .loginForm {
        padding-left: 15.5vw;
        padding-right: 15.5vw;
    }
    .loading {
        box-shadow: 2px 2px #000;
    }
</style>

<div>
    <div in:fade={{duration:100}} class="loginForm">

        <form label="Username"> 
            <input type="email" bind:value={loginInfo.username} />
        </form>

        <form label="Password"> 
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
</div>
