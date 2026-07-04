App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let name = "Tom";
	var $$exports = { ...$.legacy_api() };
	$.next();
	var text = $.text();
	text.nodeValue = "& Tom <";
	$.append($$anchor, text);
	return $.pop($$exports);
}
