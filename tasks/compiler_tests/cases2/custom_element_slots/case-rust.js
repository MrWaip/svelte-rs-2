import * as $ from "svelte/internal/client";
var root = $.from_html(`<header><slot name="actions"></slot></header> <main><slot></slot></main>`, 1);
export default function App($$anchor) {
	var fragment = root();
	$.next(2);
	$.append($$anchor, fragment);
}
customElements.define("my-layout", $.create_custom_element(App, {}, [], [], { mode: "open" }));
