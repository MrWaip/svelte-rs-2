import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let elRef = $.prop($$props, "elRef", 12, undefined);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.bind_this(div, ($$value) => elRef($$value), () => elRef());
	$.append($$anchor, div);
	return $.pop($$exports);
}
