import * as $ from "svelte/internal/client";
var root = $.from_mathml(`<math><mn>1</mn> <!></math>`);
export default function App($$anchor) {
	let content = "<mi>x</mi>";
	var math = root();
	var node = $.sibling($.child(math), 2);
	$.html(node, () => content, void 0, void 0, true);
	$.reset(math);
	$.append($$anchor, math);
}
