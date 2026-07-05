App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let name = "Tom";
	var $$exports = { ...$.legacy_api() };
	$.head("q2w0q4", ($$anchor) => {
		$.effect(() => {
			$.document.title = "& Tom <";
		});
	});
	return $.pop($$exports);
}
