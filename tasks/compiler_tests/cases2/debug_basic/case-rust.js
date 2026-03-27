App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let x = 1;
	let y = 2;
	var $$exports = { ...$.legacy_api() };
	$.template_effect(() => {
		console.log({
			x: $.snapshot(x),
			y: $.snapshot(y)
		});
		debugger;
	});
	$.template_effect(() => {
		console.log({ x: $.snapshot(x) });
		debugger;
	});
	$.template_effect(() => {
		console.log({});
		debugger;
	});
	return $.pop($$exports);
}
