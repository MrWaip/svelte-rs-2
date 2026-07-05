import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	async function fetchData() {
		// svelte-ignore await_reactivity_loss
		const result = await fetch("/api");
		return result;
	}
}
