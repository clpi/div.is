<script>
    import { slide, fade } from 'svelte/transition'
</script>

<style>
    .indH1 {
        font-size: 6rem;
    }
    h2 {
        font-size: 2rem;
    }
    .bod {
        padding-left: 15.5vw;
        padding-right: 15.5vw;
        margin: auto;
        align-content: center;
    }
    .box {
		width: 300px;
		border: 1px solid #fa4;
		border-radius: 2px;
		box-shadow: 2px 2px 8px rgba(0,0,0,0.1);
		padding: 1em;
		margin: 0 0 1em 0;
	}
</style>

<div class="bod" in:fade={{duration:100}}>
    <h2>Find user...</h2>
    <div class="box">
        <h2>Hello</h2>
        <p>world</p>
    </div>
</div>
