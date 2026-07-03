import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>x</div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let target = $.tag($.mutable_source(), "target");
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.bind_this(div, ($$value) => $.set(target, $$value), () => $.get(target));
	$.append($$anchor, div);
	return $.pop($$exports);
}
