import * as $ from "svelte/internal/client";
const row = ($$anchor, $$arg0) => {
	let c = $.derived_safe_equal(() => $.fallback($$arg0?.(), 5));
	var span = root();
	var text = $.child(span, true);
	$.reset(span);
	$.template_effect(() => $.set_text(text, $.get(c)));
	$.append($$anchor, span);
};
var root = $.from_html(`<span> </span>`);
var root_1 = $.from_html(`<button> </button> <!>`, 1);
export default function App($$anchor) {
	let count = $.state(0);
	var fragment = root_1();
	var button = $.first_child(fragment);
	var text_1 = $.child(button, true);
	$.reset(button);
	var node = $.sibling(button, 2);
	row(node, () => $.get(count));
	$.template_effect(() => $.set_text(text_1, $.get(count)));
	$.delegated("click", button, () => $.update(count));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
