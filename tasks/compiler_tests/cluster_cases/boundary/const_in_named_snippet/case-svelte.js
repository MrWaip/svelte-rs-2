import * as $ from "svelte/internal/client";
var root = $.from_html(`<p>row</p>`);
var root_1 = $.from_html(`<div><!></div>`);
export default function App($$anchor, $$props) {
	function compute() {
		return $$props.n + 1;
	}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const row = ($$anchor) => {
			const value = $.derived(compute);
			var p = root();
			$.append($$anchor, p);
		};
		$.boundary(node, {}, ($$anchor) => {
			const value = $.derived(compute);
			var div = root_1();
			var node_1 = $.child(div);
			row(node_1);
			$.reset(div);
			$.append($$anchor, div);
		});
	}
	$.append($$anchor, fragment);
}
