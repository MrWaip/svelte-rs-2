import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor) {
	var a;
	var $$promises = $.run([() => Promise.resolve(), () => a = "a"]);
	var input = root();
	$.template_effect(() => input.defaultValue = a, void 0, void 0, [$$promises[1]]);
	$.append($$anchor, input);
}
