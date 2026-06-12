import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div> </div>`);
export default function App($$anchor, $$props) {
	const $$sanitized_props = $.legacy_rest_props($$props, [
		"children",
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	const $$restProps = $.legacy_rest_props($$sanitized_props, ["bar"]);
	$.push($$props, false);
	function foo() {}
	var $$exports = { bar: foo };
	var div = root();
	$.attribute_effect(div, () => ({ ...$$restProps }));
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(($0) => $.set_text(text, $0), [() => $.untrack(foo)]);
	$.append($$anchor, div);
	$.bind_prop($$props, "bar", foo);
	return $.pop($$exports);
}
