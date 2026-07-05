App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<select></select>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let selected = $.tag($.state("a"), "selected");
	var $$exports = { ...$.legacy_api() };
	var select = root();
	$.bind_select_value(select, function get() {
		return $.get(selected);
	}, function set($$value) {
		$.set(selected, $$value);
	});
	$.append($$anchor, select);
	return $.pop($$exports);
}
