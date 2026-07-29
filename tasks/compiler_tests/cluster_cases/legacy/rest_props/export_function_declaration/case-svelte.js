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
	const $$restProps = $.legacy_rest_props($$sanitized_props, ["formatTitle", "Helper"]);
	$.push($$props, false);
	function formatTitle(prefix) {
		return prefix + "!";
	}
	class Helper {}
	var $$exports = {
		formatTitle,
		get Helper() {
			return Helper;
		},
		set Helper($$value) {
			Helper = $$value;
		}
	};
	var div = root();
	$.attribute_effect(div, () => ({ ...$$restProps }));
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(($0) => $.set_text(text, $0), [() => $.untrack(() => formatTitle("a"))]);
	$.append($$anchor, div);
	$.bind_prop($$props, "formatTitle", formatTitle);
	$.bind_prop($$props, "Helper", Helper);
	return $.pop($$exports);
}
