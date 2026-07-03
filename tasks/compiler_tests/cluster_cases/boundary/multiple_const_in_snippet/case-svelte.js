import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p>failed</p>`);
var root_2 = $.from_html(`<div> </div>`);
export default function App($$anchor, $$props) {
	function compute() {
		return $$props.n + 1;
	}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const failed = ($$anchor) => {
			const a = $.derived(compute);
			const b = $.derived(compute);
			var p = root_1();
			$.append($$anchor, p);
		};
		$.boundary(node, { failed }, ($$anchor) => {
			const a = $.derived(compute);
			const b = $.derived(compute);
			var div = root_2();
			var text = $.child(div);
			$.reset(div);
			$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}`));
			$.append($$anchor, div);
		});
	}
	$.append($$anchor, fragment);
}
