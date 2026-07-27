import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<option> </option>`);
export default function App($$anchor, $$props) {
	var option = root();
	$.attribute_effect(option, () => ({ ...$$props.rest }));
	var text = $.child(option, true);
	$.reset(option);
	$.template_effect(($0) => $.set_text(text, $0), void 0, [() => $$props.a]);
	$.append($$anchor, option);
}
