App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let a = $.tag($.state(0), "a");
	const c = $.tag($.derived(() => class {}($.get(a))), "c");
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, typeof $.get(c)));
	$.delegated("click", button, function click() {
		return $.update(a);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
