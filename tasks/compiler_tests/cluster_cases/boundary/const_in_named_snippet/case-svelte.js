import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p>row</p>`);
var root_2 = $.from_html(`<div><!></div>`);
export default function App($$anchor, $$props) {
	function compute() {
		return $$props.n + 1;
	}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const row = ($$anchor) => {
			const value = $.derived(compute);
			var p = root_1();
			$.append($$anchor, p);
		};
		$.boundary(node, {}, ($$anchor) => {
			const value = $.derived(compute);
			var div = root_2();
			var node_1 = $.child(div);
			row(node_1);
			$.reset(div);
			$.append($$anchor, div);
		});
	}
	$.append($$anchor, fragment);
}
