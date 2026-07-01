import * as $ from "svelte/internal/client";
const row = ($$anchor, $$arg0) => {
	let c = () => $$arg0?.().c;
	var span = root_1();
	var text = $.child(span, true);
	$.reset(span);
	$.template_effect(() => $.set_text(text, c()));
	$.append($$anchor, span);
};
var root_1 = $.from_html(`<span> </span>`);
var root = $.from_html(`<button> </button> <!>`, 1);
export default function App($$anchor) {
	let count = $.state(0);
	var fragment = root();
	var button = $.first_child(fragment);
	var text_1 = $.child(button, true);
	$.reset(button);
	var node = $.sibling(button, 2);
	row(node, () => ({ c: $.get(count) }));
	$.template_effect(() => $.set_text(text_1, $.get(count)));
	$.delegated("click", button, () => $.update(count));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
