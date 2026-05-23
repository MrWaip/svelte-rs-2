import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	let id = $.prop($$props, "id", 3, "");
	function getStyle() {
		return "color:red;";
	}
	var div = root();
	$.template_effect(($0) => {
		$.set_attribute(div, "data-testid", id());
		$.set_style(div, $0);
	}, [() => getStyle()]);
	$.append($$anchor, div);
}
