<script>
    import { onMount } from 'svelte';
    let exp;
    onMount(async () => {
        await fetch('http://localhost:3000/sum/10/20')
            .then(res => res.text())
            .then(body => exp = body);
    })
    

</script>
<style>
	h1, figure, p {
		text-align: center;
		margin: 0 auto;
	}

	h2 {
		font-size: 1.4em;
		text-transform: uppercase;
		font-weight: 700;
		margin: 0 0 0.5em 0;
	}

	figure {
		margin: 0 0 1em 0;
	}

	p {
		margin: 1em auto;
	}

</style>

<svelte:head>
	<title>Sapper project template</title>
</svelte:head>

<h2>memuri</h2>
<h3>{exp} sdfds</h3>

<figure>
	<figcaption>Have fun with Sapper!</figcaption>
</figure>

<p><strong>Try editing this file (src/routes/index.svelte) to test live reloading.</strong></p>
