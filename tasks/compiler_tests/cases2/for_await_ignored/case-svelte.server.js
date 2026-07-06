import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	async function process(items) {
		// svelte-ignore await_reactivity_loss
		for await (const item of items) {
			console.log(item);
		}
	}
}
