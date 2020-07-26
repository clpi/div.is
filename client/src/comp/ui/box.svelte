<script>
    import { fade } from 'svelte/transition';
    export let title;
</script>
<style>
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
    p {
    }
    .Box {
        width: 300px;
        height: 140px;
        float: left;
        /*border: 1px solid #fa4;*/
        border: 1px solid #eee;
        border-bottom-color: #fc8;
        border-bottom-width: 2px;
        border-radius: 2px;
        box-shadow: 2px 2px 2px rgba(0,0,0,0.1);
		padding: 0.8em;
        margin: 0.8em;
    }
    .Box-Orange {
        width: 300px;
        float: left;
        border: 1px solid #fa4;
        border-radius: 2px;
        box-shadow: 2px 2px 2px rgba(0,0,0,0.1);
		padding: 0.8em;
        margin: 0.8em;
    }
    .Box:hover {
        box-shadow: 2px 2px 3px rgba(0,0,0,0.15);
        background-color: #fbfafa;
        transition: box-shadow, background-color;
    }
</style>

<div class="Box" in:fade>
    {#if title!=undefined}
        <h3>{title}</h3>
    {/if}
    <p><slot></slot></p>
</div>
