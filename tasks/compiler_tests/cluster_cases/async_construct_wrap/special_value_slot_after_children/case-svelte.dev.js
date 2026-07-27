import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<select><option>--Please choose an option--</option><option> </option><option>cat</option></select>`), App[$.FILENAME], [[
	1,
	0,
	[
		[2, 1],
		[3, 1],
		[4, 1]
	]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var select = root();
	var option = $.sibling($.child(select));
	var text = $.child(option, true);
	$.reset(option);
	var option_value = {};
	$.next();
	$.reset(select);
	var select_value;
	$.init_select(select);
	$.template_effect(($0, $1, $2) => {
		$.set_text(text, $0);
		if (option_value !== (option_value = $1)) {
			option.__value = $1;
		}
		if (select_value !== (select_value = $2)) {
			select.value = (select.__value = $2) ?? "", $.select_option(select, $2);
		}
	}, void 0, [
		async () => (await $.track_reactivity_loss(Promise.resolve("dog")))(),
		async () => (await $.track_reactivity_loss(Promise.resolve("dog")))(),
		async () => (await $.track_reactivity_loss(Promise.resolve("dog")))()
	]);
	$.append($$anchor, select);
	return $.pop($$exports);
}
