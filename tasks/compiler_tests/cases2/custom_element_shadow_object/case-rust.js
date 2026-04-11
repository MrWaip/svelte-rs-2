import * as $ from "svelte/internal/client";
var root = $.from_html(`<p>dialog</p>`);
export default function App($$anchor) {
	var p = root();
	$.append($$anchor, p);
}
customElements.define("my-dialog", $.create_custom_element(App, {}, [], [], { mode: "open" }));
