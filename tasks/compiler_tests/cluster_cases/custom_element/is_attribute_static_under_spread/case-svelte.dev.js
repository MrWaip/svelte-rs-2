App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy"
]);
var root = $.add_locations($.from_html(`<button is="x-button">x</button>`, 2), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let props = $.rest_props($$props, rest_excludes, "props");
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.attribute_effect(button, () => ({ ...props }));
	$.append($$anchor, button);
	return $.pop($$exports);
}
