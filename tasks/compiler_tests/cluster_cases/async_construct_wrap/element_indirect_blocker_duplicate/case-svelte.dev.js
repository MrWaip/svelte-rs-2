import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input/>`), App[$.FILENAME], [[14, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function getValue() {
		return loaded + value;
	}
	function setValue(v) {
		value = v;
	}
	var loaded, value;
	var $$promises = $.run([async () => loaded = (await $.track_reactivity_loss(Promise.resolve(1)))(), () => value = ""]);
	var $$exports = { ...$.legacy_api() };
	var input = root();
	$.remove_input_defaults(input);
	$.template_effect(($0) => $.set_value(input, $0), [() => getValue()], void 0, [$$promises[1]]);
	$.delegated("input", input, function input_1(e) {
		return setValue(e.target.value);
	});
	$.append($$anchor, input);
	return $.pop($$exports);
}
$.delegate(["input"]);
