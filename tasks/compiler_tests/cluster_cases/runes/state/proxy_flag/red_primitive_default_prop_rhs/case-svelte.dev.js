App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let flag = $.prop($$props, "flag", 3, false);
	let value = $.tag($.state(0), "value");
	function apply() {
		$.set(value, flag());
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(value)));
	$.delegated("click", button, apply);
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
