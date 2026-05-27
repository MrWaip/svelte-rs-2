import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	let style = $.prop($$props, "style", 3, ""), size = $.prop($$props, "size", 3, "s"), label = $.prop($$props, "label", 3, "");
	var div = root();
	let styles;
	$.template_effect(() => {
		styles = $.set_style(div, style(), styles, { width: "100px" });
		$.set_class(div, 1, `box ${size() ?? ""}`);
		$.set_attribute(div, "data-label", label());
	});
	$.append($$anchor, div);
}
