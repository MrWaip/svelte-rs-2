import * as $ from "svelte/internal/client";
export async function load(cache, p) {
	return (await $.track_reactivity_loss($.assign_async(cache, "items", "??=", () => p, "src/​lib/​store.svelte.js:2:9")))();
}
