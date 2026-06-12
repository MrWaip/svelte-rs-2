import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	const $$sanitized_props = $.legacy_rest_props($$props, [
		"children",
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	const $$restProps = $.legacy_rest_props($$sanitized_props, ["foo"]);
	$.push($$props, false);
	const foo = 1;
	var $$exports = { foo };
	var div = root();
	$.attribute_effect(div, () => ({ ...$$restProps }));
	div.textContent = "1";
	$.append($$anchor, div);
	$.bind_prop($$props, "foo", foo);
	return $.pop($$exports);
}
