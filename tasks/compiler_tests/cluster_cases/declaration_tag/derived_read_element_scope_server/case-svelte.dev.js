App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div> </div> <p> </p>`, 1), App[$.FILENAME], [[1, 0], [3, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	let outer = $.tag($.derived(() => Date.now()), "outer");
	var fragment = root();
	var div = $.first_child(fragment);
	{
		let inner = $.tag($.derived(() => Date.now()), "inner");
		var text = $.child(div, true);
		$.reset(div);
		$.template_effect(() => $.set_text(text, $.get(inner)));
	}
	var p = $.sibling(div, 2);
	var text_1 = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text_1, $.get(outer)));
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
