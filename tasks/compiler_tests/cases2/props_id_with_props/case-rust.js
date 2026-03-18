import * as $ from "svelte/internal/client";
var root = $.from_html(`<div> </div>`);
export default function App($$anchor, $$props) {
	const id = $.props_id();
	var div = root();
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(() => {
		$.set_attribute(div, "id", id);
		$.set_text(text, $$props.name);
	});
	$.append($$anchor, div);
}
