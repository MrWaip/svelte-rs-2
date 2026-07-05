import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	async function process(source) {
		const results = [];
		for await (const item of $.for_await_track_reactivity_loss(source)) {
			results.push(item);
		}
		return results;
	}
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
