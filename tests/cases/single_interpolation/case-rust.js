import * as $ from "svelte/internal/client";
var root = $.template(` `, 1);
export default function App($$anchor) {
	let name = undefined;
	var fragment = root();
	var text = $.first_child(fragment, true);
	$.template_effect(() => $.set_text(text, name));
	$.append($$anchor, fragment);
}
