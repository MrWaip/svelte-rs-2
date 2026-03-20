import "svelte/internal/flags/tracing";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.tag($.state(0), "count");
	const obj = { handler() {
		return $.trace(() => "handler ((unknown):4:9)", () => {
			$.update(count);
		});
	} };
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
