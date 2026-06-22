import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<select><option>a</option><option>b</option></select>`);
export default function App($$anchor, $$props) {
	let foo = $.prop($$props, "foo", 8);
	var select = root();
	var select_value;
	$.init_select(select);
	$.template_effect(() => {
		if (select_value !== (select_value = foo())) {
			select.value = (select.__value = foo()) ?? "", $.select_option(select, foo());
		}
	});
	$.append($$anchor, select);
}
