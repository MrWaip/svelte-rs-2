App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = 0;
	var $$exports = { ...$.legacy_api() };
	$.template_effect(() => {
		console.log({ count: $.snapshot(count) });
		debugger;
	});
	return $.pop($$exports);
}
