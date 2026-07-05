import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	async function process(items) {
		// svelte-ignore await_reactivity_loss
		for await (const item of items) {
			console.log(item);
		}
	}
}
