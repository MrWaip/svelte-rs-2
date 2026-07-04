App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(` <div><br/> </div> <div></div>`, 1), App[$.FILENAME], [[
	7,
	0,
	[[8, 4]]
], [12, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const title = "world";
	var $$exports = { ...$.legacy_api() };
	$.next();
	var fragment = root();
	var text = $.first_child(fragment);
	text.nodeValue = "world ";
	var div = $.sibling(text);
	var text_1 = $.sibling($.child(div));
	text_1.nodeValue = " world";
	$.reset(div);
	var div_1 = $.sibling(div, 2);
	div_1.textContent = "world";
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
