import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input/>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let rest = $.prop($$props, "rest", 24, () => ({}));
	let value = $.prop($$props, "value", 12, "");
	var $$exports = { ...$.legacy_api() };
	var input = root();
	$.attribute_effect(input, () => ({ ...rest() }), void 0, void 0, void 0, void 0, true);
	$.bind_value(input, function get() {
		return value();
	}, function set($$value) {
		value($$value);
	});
	$.append($$anchor, input);
	return $.pop($$exports);
}
