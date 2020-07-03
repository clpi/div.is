
<script>
  import { Button, Field, Input } from 'svelma'
  import { slide, fade } from 'svelte/transition'
  let disabled = false;
  let promise = Promise.resolve([]);
  let submitted = false;
  let verifyPwd = "";
  let signupInfo = {
    email: '',
    username: '',
    password: '',
  }
  async function signupUser(userInfo) {
      const signupPost = await fetch('http://localhost:3001/api/register', {
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
      console.log(signupInfo);
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
  function verifyInput(){}
  // add name field to user model
</script>

<style>
    .signupForm{
        margin: 10vmin;
    }
    .submitAuth {
        margin-top: 3vmin;
        float: right;
        margin-right: 0vmin;
    }
</style>

<div>
    <div in:fade class="signupForm">
        <!-- Maybe do this with traditional form input instead of binding -->
        <!-- Get a form validator library -->
        <Field label="Email" type="text">
            <Input type="email" bind:value={signupInfo.email} maxlength="30" />
        </Field>
        <Field label="Username" type="text">
            <Input type="username" bind:value={signupInfo.username} />
        </Field>
        <Field label="Password"> 
            <Input type="password" bind:value={signupInfo.password} passwordReveal={true} />
        </Field>
        <Field label="Verify Password"> 
            <Input type="password" bind:value={verifyPwd} passwordReveal={true} />
        </Field>
        <div class="buttons, submitAuth">
            <Button type="is-light" nativeType="reset" on:click={handleReset}>Reset</Button>
            <Button 
                type="is-primary" 
                nativeType="submit" 
                on:click={ handleSubmit } { disabled }
                >
                Submit
            </Button>
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
