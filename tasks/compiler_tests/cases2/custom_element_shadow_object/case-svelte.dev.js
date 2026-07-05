App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p>dialog</p>`), App[$.FILENAME], [[2, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var p = root();
	$.append($$anchor, p);
	return $.pop($$exports);
}
customElements.define("my-dialog", $.create_custom_element(App, {}, [], [], {
	mode: "open",
	delegatesFocus: true
}));
