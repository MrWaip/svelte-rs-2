import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>content</div>`);
export default function App($$anchor, $$props) {
	var div = root();
	$.template_effect(() => $.set_class(div, 1, `static ${$$props.value ?? ""}`));
	$.append($$anchor, div);
}
