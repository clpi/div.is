import { writable } from "svelte/store";

let session = writable(null);
let logged = writable(false);
let duration = writable(0);

export {session, logged, duration};
