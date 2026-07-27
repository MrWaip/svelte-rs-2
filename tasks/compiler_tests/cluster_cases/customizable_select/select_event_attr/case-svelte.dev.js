App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<select id="s" class="x"><option>a</option></select>`), App[$.FILENAME], [[
	6,
	0,
	[[7, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let value = $.tag($.state("a"), "value");
	function onchange() {}
	var $$exports = { ...$.legacy_api() };
	var select = root();
	var option = $.child(select);
	option.value = option.__value = "a";
	$.reset(select);
	$.delegated("change", select, onchange);
	$.event("focus", select, function focus() {
		return $.set(value, "b");
	});
	$.bind_select_value(select, function get() {
		return $.get(value);
	}, function set($$value) {
		$.set(value, $$value);
	});
	$.append($$anchor, select);
	return $.pop($$exports);
}
$.delegate(["change"]);
