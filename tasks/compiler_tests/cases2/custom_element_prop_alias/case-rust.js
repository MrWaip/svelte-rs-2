import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	let total = $.prop($$props, "count", 3, 0);
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, total()));
	$.append($$anchor, p);
}
customElements.define("my-counter", $.create_custom_element(App, { count: {} }, [], [], { mode: "open" }));
