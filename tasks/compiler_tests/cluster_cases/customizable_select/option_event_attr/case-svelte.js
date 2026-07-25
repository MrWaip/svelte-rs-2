import * as $ from "svelte/internal/client";
var root = $.from_html(`<select><option>a</option></select>`);
export default function App($$anchor) {
	let value = $.state("a");
	function onclick() {}
	var select = root();
	var option = $.child(select);
	option.value = option.__value = "a";
	$.reset(select);
	$.delegated("click", option, onclick);
	$.bind_select_value(select, () => $.get(value), ($$value) => $.set(value, $$value));
	$.append($$anchor, select);
}
$.delegate(["click"]);
