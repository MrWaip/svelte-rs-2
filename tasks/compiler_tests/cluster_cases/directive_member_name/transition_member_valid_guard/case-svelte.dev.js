App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const fn = (node, options) => ({});
	let obj = $.tag($.derived(() => ({ inner: fn })), "obj");
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.transition(3, div, () => $.get(obj).inner);
	$.append($$anchor, div);
	return $.pop($$exports);
}
