import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	async function fetchData() {
		// svelte-ignore await_reactivity_loss
		const result = await fetch("/api");
		return result;
	}
}
