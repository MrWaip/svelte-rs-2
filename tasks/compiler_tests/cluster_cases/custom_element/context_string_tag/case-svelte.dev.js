App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy",
	"$$host"
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let props = $.rest_props($$props, rest_excludes, "props");
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
customElements.define("my-el", $.create_custom_element(App, {}, [], [], { mode: "open" }));
