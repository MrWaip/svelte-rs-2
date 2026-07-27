App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<h1>Hi</h1>`), App[$.FILENAME], [[3, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var h1 = root();
	$.append($$anchor, h1);
	return $.pop($$exports);
}
customElements.define("x-none-ext", $.create_custom_element(App, {}, [], [], void 0, (c) => c));
