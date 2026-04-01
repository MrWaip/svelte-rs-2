import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	async function fetchData() {
		// svelte-ignore await_reactivity_loss
		const result = await fetch("/api");
		return result;
	}
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
