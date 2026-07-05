App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button></button>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let focused = $.tag($.state(false), "focused");
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.bind_focused(button, function set($$value) {
		$.set(focused, $$value);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
