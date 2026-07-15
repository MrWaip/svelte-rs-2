import * as $ from "svelte/internal/client";
import { tick } from "svelte";
export async function run(p) {
	await p;
	// svelte-ignore await_reactivity_loss
	await tick();
	return 1;
}
