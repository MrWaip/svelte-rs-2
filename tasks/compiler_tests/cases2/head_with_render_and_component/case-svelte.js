import * as $ from "svelte/internal/client";
var root = $.from_html(`<style>html { height: 100%; }</style>`);
var root_1 = $.from_html(`<!> <!>`, 1);
export default function App($$anchor, $$props) {
	var fragment = root_1();
	$.head("q2w0q4", ($$anchor) => {
		var style = root();
		$.append($$anchor, style);
	});
	var node = $.first_child(fragment);
	A(node, {});
	var node_1 = $.sibling(node, 2);
	$.snippet(node_1, () => $$props.children ?? $.noop);
	$.append($$anchor, fragment);
}
