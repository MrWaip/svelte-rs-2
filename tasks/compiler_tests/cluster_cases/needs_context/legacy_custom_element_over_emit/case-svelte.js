import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<h1>hello</h1>`);
export default function App($$anchor) {
	var h1 = root();
	$.append($$anchor, h1);
}
customElements.define("my-thing", $.create_custom_element(App, {}, [], [], { mode: "open" }));
