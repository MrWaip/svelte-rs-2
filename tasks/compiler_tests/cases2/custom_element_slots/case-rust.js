import * as $ from "svelte/internal/client";
var root = $.from_html(`<header><!></header> <main><!></main>`, 1);
export default function App($$anchor, $$props) {
	var fragment = root();
	var header = $.first_child(fragment);
	var node = $.child(header);
	$.slot(node, $$props, "actions", {}, null);
	$.reset(header);
	var main = $.sibling(header, 2);
	var node_1 = $.child(main);
	$.slot(node_1, $$props, "default", {}, null);
	$.reset(main);
	$.append($$anchor, fragment);
}
customElements.define("my-layout", $.create_custom_element(App, {}, ["actions", "default"], [], { mode: "open" }));
