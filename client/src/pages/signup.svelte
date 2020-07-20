
<script>
  import { slide, fade } from 'svelte/transition'
  import { metatags } from '@sveltech/routify';
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
  async function signupUser(userInfo) {
      const signupPost = await fetch(API_URL+'/register', {
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
  function handleSubmit() {
    fetching=true;
    console.log(signupInfo);
    if (notAllowed.includes(userInfo.username)) { // TODO actually verify input
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
    .signupForm{
        padding-left: 15.5vw;
        padding-right: 15.5vw;
    }
    .submitAuth {
        margin-top: 3vmin;
        float: right;
        margin-right: 0vmin;
    }
</style>

<div>
    <div in:fade={{duration:100}} class="signupForm">
        <!-- Maybe do this with traditional form input instead of binding -->
        <!-- Get a form validator library -->
        <form label="Email" type="text">
            <input type="email" bind:value={signupInfo.email} maxlength="30" />
        </form>
        <form label="Username" type="text">
            <input type="username" bind:value={signupInfo.username} />
        </form>
        <form label="Password"> 
            <input type="password" bind:value={signupInfo.password} passwordReveal={true} />
        </form>
        <form label="Verify Password"> 
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
    </div>
</div>
