import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	var div = root();
	$.attribute_effect(div, () => ({ ...$$props.rest }));
	$.append($$anchor, div);
}
