import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<select><option>x</option></select>`), App[$.FILENAME], [[
	2,
	0,
	[[2, 18]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var a;
	var $$promises = $.run([async () => void (await $.track_reactivity_loss(Promise.resolve()))(), () => a = "a"]);
	var $$exports = { ...$.legacy_api() };
	var select = root();
	var select_value;
	$.init_select(select);
	$.template_effect(() => {
		if (select_value !== (select_value = a)) {
			select.value = select.__value = a, $.select_option(select, a);
		}
	}, void 0, void 0, [$$promises[1]]);
	$.append($$anchor, select);
	return $.pop($$exports);
}
