<script>
  import { Tabs, Tab, Button, Field, Input } from 'svelma'
  import { slide, fade } from 'svelte/transition'
</script>
<style>
    .box {
		width: 300px;
        float: left;
		border: 1px solid #fa4;
		border-radius: 2px;
		box-shadow: 2px 2px 8px rgba(0,0,0,0.1);
		padding: 1em;
		margin: 1em 1em 1em 1em;
        height: 400px;
	}
    h1 {
        font-size: 2rem;
        font-weight: 300;
    }
    h2 {
        font-size: 1.6rem;
        font-weight: 300;
    }
    h3 {
        font-size: 1.3rem;
        font-weight: 300;
    }
    Field {
        margin: 10px;
    }
    .dbhome {
        display: inline-block;
    }
</style>

<div in:fade={{duration:100}}>
    <h1>Database</h1>
    <Tabs>
        <Tab label = "Home" class="dbhome">
            <ul>
            <li class="box">
                <h3>Create user</h3>
                <Field label=Email><Input label=Email></Input></Field>             
                <Field label=Username><Input label=Username></Input></Field>             
                <Field label=Password><Input label=Passsword></Input></Field>             
                <Button>Submit</Button>
            </li>
            <li class="box">
                <h3>Create Record</h3>
                <Field label=Name><Input label=Email></Input></Field>             
                <Field label=User><Input label=Username></Input></Field>             
                <Button>Submit</Button>
            </li>
            <li class="box">
                <h3>Create Item</h3>
                <Field label=Name><Input label=Email></Input></Field>             
                <Field label=User><Input label=Username></Input></Field>             
                <Button>Submit</Button>
            </li>
            <li class="box">
                <h3>Create Field</h3>
                <Field label=Name><Input label=Email></Input></Field>             
                <Field label=Type><Input label=Username></Input></Field>             
                <Field label=Value><Input label=Username></Input></Field>             
                <Button>Submit</Button>
            </li>
        </Tab>
    <Tab label="Performance">
    </Tab>
    <Tab label="Operations"></Tab>
    </Tabs>
</div>
