import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>x</button>`);
export default function App($$anchor, $$props) {
	let theme = $.prop($$props, "theme", 3, "primary");
	var button = root();
	$.template_effect(() => $.set_class(button, 1, `ui-button ${theme() ?? ""}`));
	$.append($$anchor, button);
}
