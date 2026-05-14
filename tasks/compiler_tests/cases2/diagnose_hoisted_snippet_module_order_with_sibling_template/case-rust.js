import * as $ from "svelte/internal/client";
const defaultWrapWith = ($$anchor, mf = $.noop) => {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.snippet(node, mf);
	$.append($$anchor, fragment);
};
var root_2 = $.from_html(`<span> </span>`);
var root_3 = $.from_html(`<style>:root { --x: red; }</style>`);
var root = $.from_html(`<div><!></div>`);
export default function App($$anchor, $$props) {
	const inner = ($$anchor) => {
		var span = root_2();
		var text = $.child(span);
		$.reset(span);
		$.template_effect(() => $.set_text(text, `${label() ?? ""}0`));
		$.append($$anchor, span);
	};
	let wrapWith = $.prop($$props, "wrapWith", 3, defaultWrapWith), label = $.prop($$props, "label", 3, "");
	let count = 0;
	var div = root();
	$.head("q2w0q4", ($$anchor) => {
		var style = root_3();
		$.append($$anchor, style);
	});
	var node_1 = $.child(div);
	$.snippet(node_1, wrapWith, () => inner);
	$.reset(div);
	$.append($$anchor, div);
}
