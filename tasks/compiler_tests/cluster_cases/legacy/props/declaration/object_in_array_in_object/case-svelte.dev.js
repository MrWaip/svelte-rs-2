import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let tmp = { outer: [{ inner: 1 }] }, $$array = $.derived(() => $.to_array(tmp.outer, 1)), inner = $.prop($$props, "inner", 24, () => $.get($$array)[0].inner);
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, inner()));
	$.append($$anchor, button);
	return $.pop($$exports);
}
