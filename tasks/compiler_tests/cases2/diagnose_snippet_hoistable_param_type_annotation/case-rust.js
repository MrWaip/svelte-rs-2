import * as $ from "svelte/internal/client";
const defaultWrapWith = ($$anchor, mf = $.noop) => {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.snippet(node, mf);
	$.append($$anchor, fragment);
};
var root = $.from_html(`<div>x</div>`);
export default function App($$anchor, $$props) {
	let wrapWith = $.prop($$props, "wrapWith", 3, defaultWrapWith);
	var div = root();
	$.append($$anchor, div);
}
