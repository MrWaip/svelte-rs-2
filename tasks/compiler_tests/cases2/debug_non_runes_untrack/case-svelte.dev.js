import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let x = 1;
	var $$exports = { ...$.legacy_api() };
	$.template_effect(() => {
		console.log({ x: $.untrack(() => $.snapshot(x)) });
		debugger;
	});
	return $.pop($$exports);
}
