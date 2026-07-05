App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let foo = $.tag($.state(0), "foo");
	let bar = $.tag($.derived(() => $.get(foo) + 1), "bar");
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(foo) ?? ""} ${$.get(bar) ?? ""}`));
	$.delegated("click", button, function click() {
		return $.update(foo);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
