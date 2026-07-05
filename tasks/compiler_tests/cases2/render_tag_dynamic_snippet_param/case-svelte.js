import * as $ from "svelte/internal/client";
const wrapper = ($$anchor, inner = $.noop) => {
	var div = root();
	var node = $.child(div);
	$.snippet(node, inner);
	$.reset(div);
	$.append($$anchor, div);
};
const greeting = ($$anchor) => {
	var p = root_1();
	$.append($$anchor, p);
};
var root = $.from_html(`<div><!></div>`);
var root_1 = $.from_html(`<p>Hello</p>`);
export default function App($$anchor) {
	let msg = "hi";
	wrapper($$anchor, () => greeting);
}
