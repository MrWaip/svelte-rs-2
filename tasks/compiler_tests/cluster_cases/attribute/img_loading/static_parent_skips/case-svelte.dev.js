App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<img src="a" alt="" loading="lazy"/> <div> <img src="b" alt="" loading="lazy"/></div>`, 1), App[$.FILENAME], [[5, 0], [
	6,
	0,
	[[6, 8]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let x = 1;
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var img = $.first_child(fragment);
	var div = $.sibling(img, 2);
	var text = $.child(div, true);
	text.nodeValue = "1";
	var img_1 = $.sibling(text);
	$.reset(div);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
