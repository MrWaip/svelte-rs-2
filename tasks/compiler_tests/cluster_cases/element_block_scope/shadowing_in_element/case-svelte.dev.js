App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button> <div><b></b></div>`, 1), App[$.FILENAME], [[4, 0], [
	5,
	0,
	[[7, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let x = $.tag($.state(1), "x");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var text = $.child(button, true);
	$.reset(button);
	var div = $.sibling(button, 2);
	{
		const x = "inner";
		var b = $.child(div);
		b.textContent = "inner";
		$.reset(div);
	}
	$.template_effect(() => $.set_text(text, $.get(x)));
	$.delegated("click", button, function click() {
		return $.update(x);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
