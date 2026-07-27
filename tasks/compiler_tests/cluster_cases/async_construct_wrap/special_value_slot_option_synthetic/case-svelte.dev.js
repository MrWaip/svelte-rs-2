import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<select><option> </option></select>`), App[$.FILENAME], [[
	1,
	0,
	[[2, 2]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var select = root();
	var option = $.child(select);
	var text = $.child(option, true);
	$.reset(option);
	var option_value = {};
	$.reset(select);
	select.value = select.__value = 42, $.select_option(select, 42);
	$.init_select(select);
	$.template_effect(($0, $1) => {
		$.set_text(text, $0);
		if (option_value !== (option_value = $1)) {
			option.__value = $1;
		}
	}, void 0, [async () => (await $.track_reactivity_loss(Promise.resolve(42)))(), async () => (await $.track_reactivity_loss(Promise.resolve(42)))()]);
	$.append($$anchor, select);
	return $.pop($$exports);
}
