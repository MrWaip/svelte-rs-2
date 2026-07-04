App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let host = $$props.$$host;
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
customElements.define("my-element", $.create_custom_element(App, {}, [], [], { mode: "open" }));
