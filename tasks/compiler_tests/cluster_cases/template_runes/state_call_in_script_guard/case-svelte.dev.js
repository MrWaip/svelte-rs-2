App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let x = $.tag_proxy($.proxy({ a: 1 }), "x");
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, x.a));
	$.delegated("click", button, function click() {
		return x.a++;
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
