import * as $ from "svelte/internal/client";
var root = $.from_svg(`<a><text>Hello</text></a>`);
export default function App($$anchor) {
	var a = root();
	$.append($$anchor, a);
}
