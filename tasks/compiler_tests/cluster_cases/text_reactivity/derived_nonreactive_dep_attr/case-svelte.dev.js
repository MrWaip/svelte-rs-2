App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function load() {
		return { foo: 1 };
	}
	const c = load();
	const x = $.tag($.derived(() => c.foo), "x");
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.template_effect(() => $.set_attribute(div, "title", $.get(x)));
	$.append($$anchor, div);
	return $.pop($$exports);
}
