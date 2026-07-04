App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[4, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let el = $.tag($.state(void 0), "el");
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.bind_this(div, (v) => $.set(el, v, true), () => $.get(el));
	$.append($$anchor, div);
	return $.pop($$exports);
}
