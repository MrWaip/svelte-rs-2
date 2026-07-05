App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let invalid = $.tag($.derived(() => Boolean($$props.flag)), "invalid");
	function reset() {
		$.set(invalid, false);
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(invalid)));
	$.delegated("click", button, reset);
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
