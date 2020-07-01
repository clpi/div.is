<script>
  import { Button, Field, Input } from 'svelma'
  let loginInfo = {
    username: '',
    password: '',
  }
  let disabled = false;
  let promise = Promise.resolve([]);
  let submitted = false;
  async function loginUser(userInfo) {
      const signupPost = await fetch('http://localhost:3001/api/login', {
          method: 'POST',
          headers: {
            'content-type': 'application/json',
              /*authorization: <authorization>*/
          },
          body: JSON.stringify(userInfo)
      });
      if (signupPost.ok) {
        return "OK";
      } else {
        throw new Error(users);
      }
    }
  function handleLogin() {
    promise = loginUser(loginInfo);
    disabled = true;
    submitted = true;
  }
  // allow username OR email for signin
</script>

<style>
    .loginForm {
        margin: 1rem;
    }
</style>

<div class="loginForm">

    <Field label="Username"> 
        <Input type="email" bind:value={loginInfo.username} />
    </Field>

    <Field label="Password"> 
        <Input type="password" bind:value={loginInfo.pasword} passwordReveal={true} />
    </Field>
  <div class="buttons, submitAuth">
    <Button 
      type="is-primary" 
      nativeType="submit" 
      on:click={ handleLogin } { disabled }
    >
      Login
    </Button>
  </div>
  <div class=loginRes>
    {#await promise}
      <p>waiting...</p>
    {:then res}
      {#if submitted}
          <p>{res}</p>
          <p>Welcome {loginInfo.username}!</p>
      {/if}
    {:catch error}
      <p style="color: red">{error.message}</p>
  {/await}
  </div>

</div>
