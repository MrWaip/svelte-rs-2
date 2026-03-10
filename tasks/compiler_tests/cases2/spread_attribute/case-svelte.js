import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
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
}
