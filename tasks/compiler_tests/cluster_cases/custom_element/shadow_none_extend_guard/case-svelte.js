import * as $ from "svelte/internal/client";
var root = $.from_html(`<h1>Hi</h1>`);
export default function App($$anchor) {
	var h1 = root();
	$.append($$anchor, h1);
}
customElements.define("x-none-ext", $.create_custom_element(App, {}, [], [], void 0, (c) => c));
