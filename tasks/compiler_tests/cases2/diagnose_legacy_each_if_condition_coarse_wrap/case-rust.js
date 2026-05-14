import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_2 = $.from_html(`<p>ok</p>`);
export default function App($$anchor, $$props) {
	let rows = $.prop($$props, "rows", 8);
	function check(key) {
		return key === "a";
	}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, rows, (row) => row.key, ($$anchor, row) => {
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		{
			var consequent = ($$anchor) => {
				var p = root_2();
				$.append($$anchor, p);
			};
			var d = $.derived(() => ($.get(row), $.untrack(() => check($.get(row).key))));
			$.if(node_1, ($$render) => {
				if ($.get(d)) $$render(consequent);
			});
		}
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
