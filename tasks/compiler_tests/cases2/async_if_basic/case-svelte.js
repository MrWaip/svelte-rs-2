import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p>yes</p>`);
export default function App($$anchor) {
	async function check() {
		return true;
	}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.async(node, [], [check], (node, $$condition) => {
		var consequent = ($$anchor) => {
			var p = root_1();
			$.append($$anchor, p);
		};
		$.if(node, ($$render) => {
			if ($.get($$condition)) $$render(consequent);
		});
	});
	$.append($$anchor, fragment);
}
