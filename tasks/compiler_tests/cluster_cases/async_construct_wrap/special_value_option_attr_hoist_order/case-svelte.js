import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<option> </option>`);
export default function App($$anchor, $$props) {
	var option = root();
	var text = $.child(option, true);
	$.reset(option);
	var option_value = {};
	$.template_effect(($0, $1, $2) => {
		$.set_class(option, 1, $0);
		$.set_text(text, $1);
		if (option_value !== (option_value = $2)) {
			option.__value = $2;
		}
	}, void 0, [
		async () => $.clsx(await $$props.c),
		() => $$props.a,
		() => $$props.a
	]);
	$.append($$anchor, option);
}
