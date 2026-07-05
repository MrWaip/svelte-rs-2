import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let tmp = { x: [1] }, $$array = $.derived(() => $.to_array(tmp.x, 1)), bar = $.prop($$props, "bar", 28, () => $.get($$array)[0]);
	function inc() {
		$.update_prop(bar);
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, bar()));
	$.event("click", button, inc);
	$.append($$anchor, button);
	return $.pop($$exports);
}
