import * as $ from "svelte/internal/client";
var root = $.from_html(`<header><!></header> <main><!></main>`, 1);
export default function App($$anchor, $$props) {
	var fragment = root();
	$.next(2);
	$.append($$anchor, fragment);
}
customElements.define("my-layout", $.create_custom_element(App, {}, [], [], { mode: "open" }));
