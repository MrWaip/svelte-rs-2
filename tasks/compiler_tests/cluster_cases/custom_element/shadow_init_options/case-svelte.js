import * as $ from "svelte/internal/client";
var root = $.from_html(`<h1>Hi</h1>`);
export default function App($$anchor) {
	var h1 = root();
	$.append($$anchor, h1);
}
customElements.define("x-shadow-init", $.create_custom_element(App, {}, [], [], {
	mode: "open",
	clonable: true,
	delegatesFocus: true,
	serializable: true,
	slotAssignment: "manual"
}));
