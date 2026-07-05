App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy"
]);
var root = $.add_locations($.from_html(`<p>spread</p>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let rest = $.rest_props($$props, rest_excludes, "rest");
	var $$exports = { ...$.legacy_api() };
	var p = root();
	$.attribute_effect(p, () => ({
		...rest,
		"data-extra": "x"
	}), void 0, void 0, void 0, "svelte-qv4ee3");
	$.append($$anchor, p);
	return $.pop($$exports);
}
