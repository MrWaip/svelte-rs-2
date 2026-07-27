import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<option> </option>`);
export default function App($$anchor, $$props) {
	var option = root();
	var text = $.child(option, true);
	$.reset(option);
	var option_value = {};
	$.template_effect(($0, $1) => {
		$.set_text(text, $0);
		if (option_value !== (option_value = `x${$1 ?? ""}`)) {
			option.value = option.__value = `x${$1 ?? ""}`;
		}
	}, void 0, [() => $$props.b, () => $$props.a]);
	$.append($$anchor, option);
}
