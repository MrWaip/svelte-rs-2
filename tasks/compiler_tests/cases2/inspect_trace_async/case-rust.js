import "svelte/internal/flags/tracing";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let data = $.tag($.state(null), "data");
	async function fetchData() {
		return await $.trace(() => "fetchData ((unknown):3:1)", async () => {
			$.set(data, await fetch("/api"), true);
		});
	}
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
