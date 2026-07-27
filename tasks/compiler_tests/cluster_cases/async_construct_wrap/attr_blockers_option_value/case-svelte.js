import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<select><option>x</option></select>`);
export default function App($$anchor) {
	var a;
	var $$promises = $.run([() => Promise.resolve(), () => a = "a"]);
	var select = root();
	var option = $.child(select);
	var option_value = {};
	$.reset(select);
	$.template_effect(() => {
		if (option_value !== (option_value = a)) {
			option.value = option.__value = a;
		}
	}, void 0, void 0, [$$promises[1]]);
	$.append($$anchor, select);
}
