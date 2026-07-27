import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<select><option> </option></select>`);
export default function App($$anchor) {
	var a;
	var $$promises = $.run([() => Promise.resolve(), () => a = "a"]);
	var select = root();
	var option = $.child(select);
	var text = $.child(option, true);
	$.reset(option);
	var option_value = {};
	$.reset(select);
	var select_value;
	$.init_select(select);
	$.template_effect(($0, $1) => {
		$.set_text(text, "a");
		if (option_value !== (option_value = $0)) {
			option.value = (option.__value = $0) ?? "";
		}
		if (select_value !== (select_value = $1)) {
			select.value = (select.__value = $1) ?? "", $.select_option(select, $1);
		}
	}, void 0, [() => Promise.resolve("y"), () => Promise.resolve("x")], [$$promises[1]]);
	$.append($$anchor, select);
}
