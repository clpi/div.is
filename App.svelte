<script>
    import { onMount } from 'svelte';
	export let name;
    let expression = "";
    let userInfo = "";

    onMount(async () => {
        await fetch('http://localhost:3001/api/sum/29/69')
            .then(res => res.text())
            .then(body => expression = body)
            .catch(err=>{
                console.log(err);
            });
        await fetch('http://localhost:3001/api/user/chrisp')
            .then(res=> res.json())
            .then(body => userInfo = body)
    });
</script>

<main>
	<h1>Hello {name}!</h1>
    <h3>{expression}</h3>
    <h4>Welcome {userInfo.username}! (email: {userInfo.email})</h4>
	<p>Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn how to build Svelte apps.</p>
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	} 

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
