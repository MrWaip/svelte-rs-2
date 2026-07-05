import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[3, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	function foo(node, x) {}
	let bar = 1;
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.action(div, ($$node, $$action_arg) => foo?.($$node, $$action_arg), () => bar);
	$.append($$anchor, div);
	return $.pop($$exports);
}
