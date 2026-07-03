import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button> <!>`, 1);
export default function App($$anchor, $$props) {
	let count = $.state(0);
	var fragment = root();
	var button = $.first_child(fragment);
	var text = $.child(button, true);
	$.reset(button);
	var node = $.sibling(button, 2);
	$.snippet(node, () => $$props.children, () => $.get(count));
	$.template_effect(() => $.set_text(text, $.get(count)));
	$.delegated("click", button, () => $.update(count));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
