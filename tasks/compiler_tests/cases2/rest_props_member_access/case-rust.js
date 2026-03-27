import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p> <span> </span> <div> </div>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let props = $.rest_props($$props, [
		"$$slots",
		"$$events",
		"$$legacy",
		"id"
	]);
	const label = $.derived(() => $$props.label + "!");
	const style = $.derived(() => $$props.style);
	const color = $.derived(() => $$props.style.color);
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p, true);
	$.reset(p);
	var span = $.sibling(p, 2);
	var text_1 = $.child(span, true);
	$.reset(span);
	var div = $.sibling(span, 2);
	var text_2 = $.child(div, true);
	$.reset(div);
	$.template_effect(() => {
		$.set_text(text, $.get(label));
		$.set_text(text_1, $$props.title);
		$.set_text(text_2, $$props.nested.deep.value);
	});
	$.append($$anchor, fragment);
	$.pop();
}
