import * as $ from "svelte/internal/client";
var root = $.template(`<div></div>`);
export default function App($$anchor) {
	var div = root();
	let attributes;
	$.template_effect(() => attributes = $.set_attributes(div, attributes, {
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
