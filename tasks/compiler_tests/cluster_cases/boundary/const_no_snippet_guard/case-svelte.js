import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<div> </div>`);
export default function App($$anchor, $$props) {
	function compute() {
		return $$props.n + 1;
	}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.boundary(node, {}, ($$anchor) => {
		const value = $.derived(compute);
		var div = root_1();
		var text = $.child(div, true);
		$.reset(div);
		$.template_effect(() => $.set_text(text, $.get(value)));
		$.append($$anchor, div);
	});
	$.append($$anchor, fragment);
}
