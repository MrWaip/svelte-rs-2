App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<header><!></header> <main><!></main>`, 1), App[$.FILENAME], [[2, 0], [3, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
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
	return $.pop($$exports);
}
customElements.define("my-layout", $.create_custom_element(App, {}, ["actions", "default"], [], { mode: "open" }));
