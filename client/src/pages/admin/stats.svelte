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
	}
    h1 {
        font-size: 2rem;
        font-weight: 300;
    }
    h1 {
        font-size: 1.6rem;
        font-weight: 300;
    }
    h3 {
        font-size: 1.3rem;
        font-weight: 300;
    }
</style>

<div in:fade={{duration:100}}>
    <h1>Statistics</h1>
    <Tabs>
        <Tab label = "Users">
            <h2>Create users/<h2>
        </Tab>
    <Tab label="Traffic">
    </Tab>
    <Tab label="Performance"></Tab>
    </Tabs>
</div>

