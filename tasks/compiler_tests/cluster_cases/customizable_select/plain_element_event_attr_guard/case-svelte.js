import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div> <button class="y">b</button>`, 1);
export default function App($$anchor) {
	let rest = $.proxy({});
	function onclick() {}
	var fragment = root();
	var div = $.first_child(fragment);
	$.attribute_effect(div, () => ({
		...rest,
		onclick,
		class: "x"
	}));
	var button = $.sibling(div, 2);
	$.delegated("click", button, onclick);
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
