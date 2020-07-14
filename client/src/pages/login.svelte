<script>
  import { Button, Field, Input } from 'svelma'
  import { slide, fade }from 'svelte/transition'
  import { setContext, getContext, onMount } from 'svelte'

  let loginInfo = {
    username: '',
    password: '',
  }
  export async function logged() {
    const res = await fetch('http://localhost:3001/api/userstatus', {
              method: 'GET',
              credentials: 'include',
              headers: {
                cookie: document.cookie,
              }
          });
          if (!res.ok) {
            setContext('loggedIn', false);
            setContext('userData', null);
              loggedIn = false;
              return null;
          } else {
              return session;
          }

  }
  let fetching = false;
  let disabled = false;
  let promise = Promise.resolve([]);
  let submitted = false;
  async function loginUser(userInfo) {
      const loginPost = await fetch('http://localhost:3001/api/login', {
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
        return loginPost;
      } else {
        throw new Error(users);
      }
    }
  function handleLogin() {
    fetching = true;
    promise = loginUser(loginInfo);
    disabled = true;
    submitted = true;
  }
  // allow username OR email for signin
</script>

<style>
    .loginForm {
        padding-left: 15.5vw;
        padding-right: 15.5vw;
    }
</style>

<div>
    <div in:fade={{duration:100}} class="loginForm">

        <Field label="Username"> 
            <Input type="email" bind:value={loginInfo.username} />
        </Field>

        <Field label="Password"> 
            <Input type="password" bind:value={loginInfo.password} passwordReveal={true} />
        </Field>
        <div class="buttons, submitAuth">
            <Button 
                nativeType="submit" 
                class={fetching ? 'is-primary is-loading' : 'is-primary'}
                on:click={ handleLogin } { disabled }
                >
                Login
            </Button>
        </div>
        <div class=loginRes>
            {#await promise}
                <p>waiting...</p>
            {:then userData}
                {#if submitted}
                    <p>Welcome {loginInfo.username}!</p>
                    <p>Your ID: {loginInfo.uid}</p>
                {/if}
            {:catch error}
                <p style="color: red">{error.message}</p>
            {/await}
        </div>

    </div>
</div>
