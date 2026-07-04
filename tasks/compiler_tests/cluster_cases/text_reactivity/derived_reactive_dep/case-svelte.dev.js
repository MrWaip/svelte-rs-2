App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<h1> </h1>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let s = $.tag($.state(0), "s");
	function inc() {
		$.update(s);
	}
	const x = $.tag($.derived(() => $.get(s) + 1), "x");
	var $$exports = { ...$.legacy_api() };
	var h1 = root();
	var text = $.child(h1, true);
	$.reset(h1);
	$.template_effect(() => $.set_text(text, $.get(x)));
	$.append($$anchor, h1);
	return $.pop($$exports);
}
