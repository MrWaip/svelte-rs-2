import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$ownership_validator = $.create_ownership_validator($$props);
	let container = $.prop($$props, "container", 28, () => ({}));
	let paths = $.prop($$props, "paths", 24, () => ["a"]);
	var $$exports = { ...$.legacy_api() };
	$.init();
	var div = root();
	$.bind_this(div, ($$value) => container(container()[paths()[0]] = $$value, true), () => container()?.[paths()[0]]);
	$.append($$anchor, div);
	return $.pop($$exports);
}
