import "svelte/internal/flags/tracing";
import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let data = $.state(null);
	async function fetchData() {
		return await $.trace(() => "fetchData (3:1)", async () => {
			$.set(data, await fetch("/api"), true);
		});
	}
}
