import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="checkbox"/>`), App[$.FILENAME], [[2, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let g = $.tag($.state(void 0), "g");
	var $$exports = { ...$.legacy_api() };
	var input = root();
	$.remove_input_defaults(input);
	var input_value;
	$.template_effect(($0) => {
		if (input_value !== (input_value = $0)) {
			input.value = (input.__value = $0) ?? "";
		}
	}, void 0, [async () => (await $.track_reactivity_loss($$props.a))()]);
	$.bind_checked(input, function get() {
		return $.get(g);
	}, function set($$value) {
		$.set(g, $$value);
	});
	$.append($$anchor, input);
	return $.pop($$exports);
}
