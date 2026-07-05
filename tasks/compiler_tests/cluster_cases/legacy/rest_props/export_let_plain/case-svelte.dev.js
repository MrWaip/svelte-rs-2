import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	const $$sanitized_props = $.legacy_rest_props($$props, [
		"children",
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	const $$restProps = $.legacy_rest_props($$sanitized_props, ["foo"]);
	$.push($$props, false, App);
	let foo = $.prop($$props, "foo", 8, 1);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.attribute_effect(div, () => ({ ...$$restProps }));
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(() => $.set_text(text, foo()));
	$.append($$anchor, div);
	return $.pop($$exports);
}
