import * as $ from "svelte/internal/client";
var root = $.from_html(`<select></select>`);
export default function App($$anchor) {
	function test() {}
	var select = root();
	var select_value;
	$.init_select(select);
	$.template_effect(($0) => {
		if (select_value !== (select_value = $0)) {
			select.value = (select.__value = $0) ?? "", $.select_option(select, $0);
		}
	}, [() => test()]);
	$.append($$anchor, select);
}
