import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	const $$sanitized_props = $.legacy_rest_props($$props, [
		"children",
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	const $$restProps = $.legacy_rest_props($$sanitized_props, ["formatTitle", "Helper"]);
	$.push($$props, false, App);
	function formatTitle(prefix) {
		return prefix + "!";
	}
	class Helper {}
	var $$exports = {
		...$.legacy_api(),
		get formatTitle() {
			return formatTitle;
		},
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
