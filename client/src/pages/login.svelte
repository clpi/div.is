<script>
  import { Field, Input } from 'svelma'
  let bound = {
    username: '',
    password: '',
  }
  // allow username OR email for signin
</script>

<style>
    .loginForm {
        margin: 10vmin;
    }
</style>

<div class="loginForm">

    <Field label="Username"> 
        <Input type="email" bind:value={bound.username} />
    </Field>

    <Field label="Password"> 
        <Input type="password" bind:value={bound.password} passwordReveal={true} />
    </Field>

</div>
