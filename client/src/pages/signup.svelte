
<script>
  import { Button, Field, Input } from 'svelma'
  let bound = {
    email: '',
    username: '',
    password: '',
  }
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

<div class="signupForm">
    <Field label="Email" type="text" >
        <Input type="email" bind:value={bound.email} maxlength="30" />
    </Field>

    <Field label="Username" type="text">
        <Input type="email" bind:value={bound.username} />
    </Field>

    <Field label="Password"> 
        <Input type="password" bind:value={bound.password} passwordReveal={true} />
    </Field>
    <div class="buttons, submitAuth">
        <Button type="is-light" nativeType="reset">Reset</Button>
        <Button type="is-primary" nativeType="submit" on:click()>Submit</Button>
    </div>
    <p> {bound.email}, {bound.username}, {bound.password}</p>
</div>
