App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input/>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let extra = $.prop($$props, "extra", 19, () => ({}));
	var $$exports = { ...$.legacy_api() };
	var input = root();
	$.attribute_effect(input, () => ({ ...extra() }), void 0, void 0, void 0, void 0, true);
	$.append($$anchor, input);
	return $.pop($$exports);
}
