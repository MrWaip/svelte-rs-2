App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let value = $.tag($.state(0), "value");
	function inc() {
		$.set(value, $.get(value) + 1);
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(value)));
	$.delegated("click", button, inc);
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
