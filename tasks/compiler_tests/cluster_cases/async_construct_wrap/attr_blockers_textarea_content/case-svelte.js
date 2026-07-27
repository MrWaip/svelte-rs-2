import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<textarea></textarea>`);
export default function App($$anchor) {
	var value;
	var $$promises = $.run([() => Promise.resolve(), () => value = "value"]);
	var textarea = root();
	$.remove_textarea_child(textarea);
	$.template_effect(() => $.set_value(textarea, value), void 0, void 0, [$$promises[1]]);
	$.append($$anchor, textarea);
}
