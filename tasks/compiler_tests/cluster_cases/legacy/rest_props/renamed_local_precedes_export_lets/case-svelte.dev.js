import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	const $$sanitized_props = $.legacy_rest_props($$props, [
		"children",
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	const $$restProps = $.legacy_rest_props($$sanitized_props, [
		"class",
		"data",
		"scale"
	]);
	$.push($$props, false, App);
	let className = $.prop($$props, "class", 8, "");
	let data = $.prop($$props, "data", 8);
	let scale = $.prop($$props, "scale", 8, 1);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.attribute_effect(div, () => ({
		...$$restProps,
		class: className()
	}));
	var text = $.child(div);
	$.reset(div);
	$.template_effect(() => $.set_text(text, `${data() ?? ""}${scale() ?? ""}`));
	$.append($$anchor, div);
	return $.pop($$exports);
}
