import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	let obj = $.mutable_source();
	function upd(e) {
		$.set(obj, e);
	}
	var div = root();
	let styles;
	$.template_effect(() => styles = $.set_style(div, "", styles, { width: `${($.get(obj), $.untrack(() => ($.get(obj)?.w || 0) + 40)) ?? ""}px` }));
	$.event("click", div, upd);
	$.append($$anchor, div);
}
