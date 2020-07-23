import { writable } from 'svelte/store';

export let user = writable({});
export let isLogged = writable(false);
export let logged = writable({});
