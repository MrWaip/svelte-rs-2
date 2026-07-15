App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<select><option>a</option></select>`), App[$.FILENAME], [[
	6,
	0,
	[[7, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let ref;
	let val = $.tag($.state("a"), "val");
	var $$exports = { ...$.legacy_api() };
	var select = root();
	$.bind_this(select, ($$value) => ref = $$value, () => ref);
	$.bind_select_value(select, function get() {
		return $.get(val);
	}, function set($$value) {
		$.set(val, $$value);
	});
	$.append($$anchor, select);
	return $.pop($$exports);
}
