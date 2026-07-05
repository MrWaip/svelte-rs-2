App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>inc</button> <div><span> </span></div>`, 1), App[$.FILENAME], [[5, 0], [
	6,
	0,
	[[6, 5]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.tag($.state(0), "count");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var div = $.sibling(button, 2);
	var span = $.child(div);
	var text = $.child(span, true);
	$.reset(span);
	$.reset(div);
	$.template_effect(() => $.set_text(text, $.get(count)));
	$.delegated("click", button, function click() {
		return $.update(count);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
