import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<select><option>x</option></select>`);
export default function App($$anchor) {
	var a;
	var $$promises = $.run([() => Promise.resolve(), () => a = "a"]);
	var select = root();
	var select_value;
	$.init_select(select);
	$.template_effect(() => {
		if (select_value !== (select_value = a)) {
			select.value = select.__value = a, $.select_option(select, a);
		}
	}, void 0, void 0, [$$promises[1]]);
	$.append($$anchor, select);
}
