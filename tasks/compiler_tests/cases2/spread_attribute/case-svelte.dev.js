App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[2, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.attribute_effect(div, () => ({
		visible: true,
		title: `idx: ${idx ?? ""}`,
		test,
		i18n,
		positive: true,
		...props,
		id: "unique",
		...rest
	}));
	$.append($$anchor, div);
	return $.pop($$exports);
}
