import * as $ from "svelte/internal/client";
var root = $.from_html(`<p class="svelte-17bjde4">styled</p>`);
export default function App($$anchor) {
	var p = root();
	$.append($$anchor, p);
}
customElements.define("my-styled", $.create_custom_element(App, {}, [], [], { mode: "open" }));
