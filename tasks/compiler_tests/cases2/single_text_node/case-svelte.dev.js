App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.next();
	var text = $.text("some long text line");
	$.append($$anchor, text);
	return $.pop($$exports);
}
