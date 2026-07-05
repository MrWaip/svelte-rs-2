import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let url = "/api";
	function outer() {
		async function inner() {
			let $$d = await $.async_derived(async () => (await $.track_reactivity_loss(fetch(url).then((r) => r.json())))(), "[$derived object]", "(unknown):6:24"), data = $.tag($.derived(() => $.get($$d).data), "data"), meta = $.tag($.derived(() => $.get($$d).meta), "meta");
			return 1;
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
