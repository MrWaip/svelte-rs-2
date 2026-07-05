App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let x = $.tag($.state(1), "x");
	let y = $.tag($.state(2), "y");
	function inc() {
		$.update(x);
		$.update(y);
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(x) ?? ""}${$.get(y) ?? ""}`));
	$.delegated("click", button, inc);
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
