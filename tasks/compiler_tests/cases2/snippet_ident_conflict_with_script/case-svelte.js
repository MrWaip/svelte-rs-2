import * as $ from "svelte/internal/client";
const card = ($$anchor, heading = $.noop) => {
	var div = root_1();
	var h3 = $.child(div);
	var text = $.child(h3, true);
	$.reset(h3);
	var node_1 = $.sibling(h3, 2);
	$.snippet(node_1, () => badge, () => "new");
	$.reset(div);
	$.template_effect(() => $.set_text(text, heading()));
	$.append($$anchor, div);
};
var root_1 = $.from_html(`<div><h3> </h3> <!></div>`);
export default function App($$anchor) {
	function action(node, arg) {
		return { destroy() {} };
	}
}
