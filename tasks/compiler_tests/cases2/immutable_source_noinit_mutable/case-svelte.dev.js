import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>hi</div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let el = $.tag($.mutable_source(), "el");
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.bind_this(div, ($$value) => $.set(el, $$value), () => $.get(el));
	$.append($$anchor, div);
	return $.pop($$exports);
}
