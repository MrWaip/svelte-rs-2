import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let url = "/api";
	function outer() {
		async function inner() {
			let data = await $.async_derived(async () => (await $.track_reactivity_loss(fetch(url)))(), "data", "(unknown):6:14");
			return $.get(data);
		}
		return inner;
	}
	var $$exports = {
		...$.legacy_api(),
		get outer() {
			return outer;
		}
	};
	return $.pop($$exports);
}
