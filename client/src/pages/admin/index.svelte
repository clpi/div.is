<script>
  import { slide, fade } from 'svelte/transition'
  import { Button } from 'svelma'
</script>

<style>
    .admin-index {
        padding-left: 15.5vw;
        padding-right: 15.5vw;
    }
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
    h2 {
        font-size: 1.6rem;
        font-weight: 300;
    }
    h3 {
        font-size: 1.3rem;
        font-weight: 300;
    }
</style>

<div class="admin-index" in:fade={{duration:100}}>
    <h1>Admin Dashboard</h1>
    <div class="box">
        <h3>Admin</h3>
        <p>Hello</p>
    </div> 
        <Button 
            type="is-primary" 
            nativeType="submit" 
            on:click={ console.log("hello") }
            >
            Press
        </Button>
</div>
