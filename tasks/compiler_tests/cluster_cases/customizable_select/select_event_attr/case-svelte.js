import * as $ from "svelte/internal/client";
var root = $.from_html(`<select id="s" class="x"><option>a</option></select>`);
export default function App($$anchor) {
	let value = $.state("a");
	function onchange() {}
	var select = root();
	var option = $.child(select);
	option.value = option.__value = "a";
	$.reset(select);
	$.delegated("change", select, onchange);
	$.event("focus", select, () => $.set(value, "b"));
	$.bind_select_value(select, () => $.get(value), ($$value) => $.set(value, $$value));
	$.append($$anchor, select);
}
$.delegate(["change"]);
