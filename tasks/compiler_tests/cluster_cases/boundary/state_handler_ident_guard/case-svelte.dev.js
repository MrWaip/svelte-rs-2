App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let handler = (e) => console.error(...$.log_if_contains_state("error", e));
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.boundary(node, { get onerror() {
		return handler;
	} }, ($$anchor) => {
		$.next();
		var text = $.text("x");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
