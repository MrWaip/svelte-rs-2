App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { fade, fly } from "svelte/transition";
var root = $.add_locations($.from_html(`<div><div></div></div>`), App[$.FILENAME], [[
	6,
	0,
	[[7, 4]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const handler = () => {};
	var $$exports = { ...$.legacy_api() };
	var div = root();
	var div_1 = $.child(div);
	$.reset(div);
	$.delegated("click", div, handler);
	$.delegated("click", div_1, handler);
	$.transition(1, div_1, () => fly);
	$.transition(2, div, () => fade);
	$.append($$anchor, div);
	return $.pop($$exports);
}
$.delegate(["click"]);
