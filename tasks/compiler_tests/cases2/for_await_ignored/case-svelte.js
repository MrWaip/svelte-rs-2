import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	async function process(items) {
		// svelte-ignore await_reactivity_loss
		for await (const item of items) {
			console.log(item);
		}
	}
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
