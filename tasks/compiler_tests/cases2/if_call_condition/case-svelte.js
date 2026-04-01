import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p>even</p>`);
var root_2 = $.from_html(`<p>odd</p>`);
export default function App($$anchor) {
	let count = 0;
	function is_even() {
		return count % 2 === 0;
	}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var p = root_1();
			$.append($$anchor, p);
		};
		var d = $.derived(() => is_even());
		var alternate = ($$anchor) => {
			var p_1 = root_2();
			$.append($$anchor, p_1);
		};
		$.if(node, ($$render) => {
			if ($.get(d)) $$render(consequent);
			else $$render(alternate, -1);
		});
	}
	$.append($$anchor, fragment);
}
