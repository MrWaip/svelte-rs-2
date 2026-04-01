import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	let props = $.proxy({
		id: "a",
		class: "from-spread"
	});
	let active = true;
	var div = root();
	$.attribute_effect(div, () => ({ ...props }));
	$.set_class(div, 1, "", null, {}, { active });
	$.append($$anchor, div);
}
