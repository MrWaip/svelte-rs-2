import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor) {
	function getValue() {
		return loaded + value;
	}
	function setValue(v) {
		value = v;
	}
	var loaded, value;
	var $$promises = $.run([async () => loaded = await Promise.resolve(1), () => value = ""]);
	var input = root();
	$.remove_input_defaults(input);
	$.template_effect(($0) => $.set_value(input, $0), [() => getValue()], void 0, [$$promises[1]]);
	$.delegated("input", input, (e) => setValue(e.target.value));
	$.append($$anchor, input);
}
$.delegate(["input"]);
