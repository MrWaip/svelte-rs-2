import * as $ from "svelte/internal/client";
const greeting = ($$anchor, $$arg0) => {
	let name = () => $$arg0?.().name;
	let age = () => $$arg0?.().age;
	var p = root_1();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${name() ?? ""} is ${age() ?? ""}`));
	$.append($$anchor, p);
};
const withDefault = ($$anchor, $$arg0) => {
	let label = $.derived_safe_equal(() => $.fallback($$arg0?.().label, "default"));
	var span = root_2();
	var text_1 = $.child(span, true);
	$.reset(span);
	$.template_effect(() => $.set_text(text_1, $.get(label)));
	$.append($$anchor, span);
};
const withRest = ($$anchor, $$arg0) => {
	let id = () => $$arg0?.().id;
	let rest = () => $.exclude_from_object($$arg0?.(), ["id"]);
	var div = root_3();
	var text_2 = $.child(div, true);
	$.reset(div);
	$.template_effect(() => $.set_text(text_2, id()));
	$.append($$anchor, div);
};
var root_1 = $.from_html(`<p> </p>`);
var root_2 = $.from_html(`<span> </span>`);
var root_3 = $.from_html(`<div> </div>`);
var root = $.from_html(`<!> <!> <!>`, 1);
export default function App($$anchor) {
	let data = $.proxy({
		name: "world",
		age: 25
	});
	var fragment = root();
	var node = $.first_child(fragment);
	greeting(node, () => data);
	var node_1 = $.sibling(node, 2);
	withDefault(node_1, () => ({}));
	var node_2 = $.sibling(node_1, 2);
	withRest(node_2, () => ({
		id: 1,
		extra: true
	}));
	$.append($$anchor, fragment);
}
