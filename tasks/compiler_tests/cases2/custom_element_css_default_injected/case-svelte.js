import * as $ from "svelte/internal/client";
var root = $.from_html(`<p class="svelte-17bjde4">styled</p>`);
const $$css = {
	hash: "svelte-17bjde4",
	code: "p.svelte-17bjde4 {color:red;}"
};
export default function App($$anchor) {
	$.append_styles($$anchor, $$css);
	var p = root();
	$.append($$anchor, p);
}
customElements.define("my-styled", $.create_custom_element(App, {}, [], [], { mode: "open" }));
