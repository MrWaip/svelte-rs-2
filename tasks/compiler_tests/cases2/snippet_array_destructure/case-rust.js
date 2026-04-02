import * as $ from "svelte/internal/client";
const show = ($$anchor, $$arg0) => {
	var $$array = $.derived(() => $.to_array($$arg0?.(), 2));
	let a = () => $.get($$array)[0];
	let b = () => $.get($$array)[1];
	var p = root_1();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${a() ?? ""} and ${b() ?? ""}`));
	$.append($$anchor, p);
};
const withRest = ($$anchor, $$arg0) => {
	var $$array_1 = $.derived(() => $.to_array($$arg0?.()));
	let first = () => $.get($$array_1)[0];
	let others = () => $.get($$array_1).slice(1);
	var p_1 = root_2();
	var text_1 = $.child(p_1, true);
	$.reset(p_1);
	$.template_effect(() => $.set_text(text_1, first()));
	$.append($$anchor, p_1);
};
var root_1 = $.from_html(`<p> </p>`);
var root_2 = $.from_html(`<p> </p>`);
var root = $.from_html(`<!> <!>`, 1);
export default function App($$anchor) {
	let pair = $.proxy([10, 20]);
	var fragment = root();
	var node = $.first_child(fragment);
	show(node, () => pair);
	var node_1 = $.sibling(node, 2);
	withRest(node_1, () => [
		1,
		2,
		3
	]);
	$.append($$anchor, fragment);
}
