App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[4, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let p = $.tag($.state(null), "p");
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(p)));
	$.delegated("click", button, function click() {
		return $.set(p, new Promise(() => {}), true);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
