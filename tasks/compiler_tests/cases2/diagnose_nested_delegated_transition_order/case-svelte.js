import * as $ from "svelte/internal/client";
import { fade, fly } from "svelte/transition";
var root = $.from_html(`<div><div></div></div>`);
export default function App($$anchor) {
	const handler = () => {};
	var div = root();
	var div_1 = $.child(div);
	$.reset(div);
	$.delegated("click", div, handler);
	$.delegated("click", div_1, handler);
	$.transition(1, div_1, () => fly);
	$.transition(2, div, () => fade);
	$.append($$anchor, div);
}
$.delegate(["click"]);
