App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<textarea></textarea>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let value = $.prop($$props, "value", 15, ""), extra = $.prop($$props, "extra", 19, () => ({}));
	var $$exports = { ...$.legacy_api() };
	var textarea = root();
	$.remove_textarea_child(textarea);
	$.attribute_effect(textarea, () => ({ ...extra() }));
	$.bind_value(textarea, function get() {
		return value();
	}, function set($$value) {
		value($$value);
	});
	$.append($$anchor, textarea);
	return $.pop($$exports);
}
