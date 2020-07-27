
<script>
  import { fade } from 'svelte/transition'
  import { metatags } from '@sveltech/routify';
  import ContentBg from '../comp/ui/contentBg.svelte';
  metatags.title = "signup";
  let disabled = false;
  let promise = Promise.resolve([]);
  let submitted = false;
  let fetching = false;
  let verifyPwd = "";
  let signupInfo = {
    email: '',
    username: '',
    password: '',
  }
  let notAllowed = ["about", "contact", "index", "login", "signup", "dashboard", "admin"];
  async function signupUser(loginInfo) {
      const signupPost = await fetch(API_URL+'/auth/register', {
          method: 'POST',
          headers: {
            'content-type': 'application/json',
              /*authorization: <authorization>*/
          },
          body: JSON.stringify(loginInfo)
      });
      if (signupPost.ok) {
        return "OK";
      } else {
        throw new Error(users);
      }
    }
  function handleSubmit() {
    fetching=true;
    console.log(signupInfo);
    if (notAllowed.includes(signupInfo.username)) { // TODO actually verify input
        throw new Error("Can't use username " + userInfo.username);
    }
    promise = signupUser(signupInfo);
    disabled = true;
    submitted = true;
  }

  function handleReset() {
    signupInfo.email = "";
    signupInfo.username = "";
    signupInfo.password = "";
  }

  // maybe get a library for this
  function verifyinput(){}
  // add name form to user model
</script>

<style>
    .signup {
      padding-left: 10vw;
      padding-right: 10vw;
    }
    .signup-head {
      display: inline-block;
      margin: auto;
      text-align: center;
    }
    .signup-left{
      padding-left: 10%;
      padding-bottom: 10%;
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
    .submitAuth {
        margin-top: 3vmin;
        float: left;
        margin-right: 0vmin;
    }
</style>

<div class="signup">
    <ContentBg>
      <div class="signup-left">
      <div class="signup-head">
        <h1 class="title" in:fade={{duration: 600}}>Welcome on board! </h1>
      </div>
        <form label="Email" type="text">
            <h3>Email</h3>
            <input type="email" bind:value={signupInfo.email} maxlength="30" />
            <h3>Username</h3>
            <input type="username" bind:value={signupInfo.username} />
            <h3>Password</h3>
            <input type="password" bind:value={signupInfo.password} passwordReveal={true} />
            <h3>Verify Password</h3>
            <input type="password" bind:value={verifyPwd} passwordReveal={true} />
        </form>
        <div class="buttons, submitAuth">
            <button type="is-light" nativeType="reset" on:click={handleReset}>Reset</button>
            <button 
                class={fetching? "is-primary is-loading" : "is-primary" }
                nativeType="submit" 
                on:click={ handleSubmit } { disabled }
                >
                Submit
            </button>
        </div>
        <div class=loginRes>
            {#await promise}
                <p>waiting...</p>
            {:then}
                {#if submitted}
                    <p>Created {signupInfo.username}! Now <a href="/login">login</a>.</p>
                {/if}
            {:catch error}
                <p style="color: red">{error.message}</p>
            {/await}
        </div>
    </ContentBg>
</div>
