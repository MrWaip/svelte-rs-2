import * as $ from "svelte/internal/client";
var root = $.from_mathml(`<math><mi>x</mi></math>`);
export default function App($$anchor) {
	var math = root();
	$.append($$anchor, math);
}
