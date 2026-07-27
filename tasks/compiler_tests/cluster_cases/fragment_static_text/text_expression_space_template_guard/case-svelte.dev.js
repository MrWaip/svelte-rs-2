import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	$.next();
	var text = $.text();
	text.nodeValue = `A${x ?? ""}B`;
	$.append($$anchor, text);
	return $.pop($$exports);
}
