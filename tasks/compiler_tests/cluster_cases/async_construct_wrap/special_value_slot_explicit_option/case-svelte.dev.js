import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<select><option> </option></select>`), App[$.FILENAME], [[
	2,
	0,
	[[3, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var a;
	var $$promises = $.run([async () => void (await $.track_reactivity_loss(Promise.resolve()))(), () => a = "a"]);
	var $$exports = { ...$.legacy_api() };
	var select = root();
	var option = $.child(select);
	var text = $.child(option, true);
	$.reset(option);
	var option_value = {};
	$.reset(select);
	var select_value;
	$.init_select(select);
	$.template_effect(($0, $1) => {
		$.set_text(text, "a");
		if (option_value !== (option_value = $0)) {
			option.value = (option.__value = $0) ?? "";
		}
		if (select_value !== (select_value = $1)) {
			select.value = (select.__value = $1) ?? "", $.select_option(select, $1);
		}
	}, void 0, [async () => (await $.track_reactivity_loss(Promise.resolve("y")))(), async () => (await $.track_reactivity_loss(Promise.resolve("x")))()], [$$promises[1]]);
	$.append($$anchor, select);
	return $.pop($$exports);
}
