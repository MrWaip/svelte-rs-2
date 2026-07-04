import * as $ from "svelte/internal/client";
var root = $.from_html(`<p>failed</p>`);
var root_1 = $.from_html(`<div> </div>`);
export default function App($$anchor, $$props) {
	function compute() {
		return $$props.n + 1;
	}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const failed = ($$anchor) => {
			const value = $.derived(compute);
			var p = root();
			$.append($$anchor, p);
		};
		$.boundary(node, { failed }, ($$anchor) => {
			const value = $.derived(compute);
			var div = root_1();
			var text = $.child(div, true);
			$.reset(div);
			$.template_effect(() => $.set_text(text, $.get(value)));
			$.append($$anchor, div);
		});
	}
	$.append($$anchor, fragment);
}
