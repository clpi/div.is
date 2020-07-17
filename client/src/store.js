import { writable } from 'svelte/store';

export let user = writable({});
export let logged = writable(false);
