import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p>ok</p>`), App[$.FILENAME], [[10, 24]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let rows = $.prop($$props, "rows", 8);
	function check(key) {
		return $.strict_equals(key, "a");
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, rows, (row) => row.key, ($$anchor, row) => {
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		{
			var consequent = ($$anchor) => {
				var p = root();
				$.append($$anchor, p);
			};
			var d = $.derived(() => ($.get(row), $.untrack(() => check($.get(row).key))));
			$.add_svelte_meta(() => $.if(node_1, ($$render) => {
				if ($.get(d)) $$render(consequent);
			}), "if", App, 10, 4);
		}
		$.append($$anchor, fragment_1);
	}), "each", App, 9, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
