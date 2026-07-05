App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let state = $.tag($.state(0), "state");
	let derived = $.tag($.derived(() => $.get(state) + 1), "derived");
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(state) ?? ""} ${$.get(derived) ?? ""}`));
	$.delegated("click", button, function click() {
		return $.update(state);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
