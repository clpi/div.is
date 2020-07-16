<script>
    import { fade } from 'svelte/transition';
    export let title;
</script>
<style>
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
    .Box {
        width: 300px;
        float: left;
        border: 1px solid #fa4;
        border-radius: 2px;
        border-bottom: 1px solid #fa4;
        box-shadow: 1px 2px 8px rgba(0,0,0,0.1);
		padding: 0.8em;
        margin: 0.8em;
    }
    .Box:hover {
        box-shadow: 1px 2px 8px rgba(0,0,0,0.15);
        transition: box-shadow;
    }
</style>

<div class="Box" in:fade>
    {#if title!=undefined}
        <h3>{title}</h3>
    {/if}
    <p><slot></slot></p>
</div>
