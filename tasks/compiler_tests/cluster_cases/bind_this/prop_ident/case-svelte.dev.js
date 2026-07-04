import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[4, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let foo = $.prop($$props, "foo", 12);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.bind_this(div, ($$value) => foo($$value), () => foo());
	$.append($$anchor, div);
	return $.pop($$exports);
}
