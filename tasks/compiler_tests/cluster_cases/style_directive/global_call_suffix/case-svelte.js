import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>go</button> <div></div>`, 1);
export default function App($$anchor) {
	let x = $.state(0);
	let w = $.state(0);
	var fragment = root();
	var button = $.first_child(fragment);
	var div = $.sibling(button, 2);
	let styles;
	$.template_effect(($0) => styles = $.set_style(div, "", styles, $0), [() => ({ left: `${Math.min($.get(x) + 3, $.get(w) - 10)}px` })]);
	$.delegated("click", button, () => {
		$.update(x);
		$.update(w);
	});
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
