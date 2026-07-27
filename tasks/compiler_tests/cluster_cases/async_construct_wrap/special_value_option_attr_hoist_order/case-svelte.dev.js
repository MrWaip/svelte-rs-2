import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<option> </option>`), App[$.FILENAME], [[2, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var option = root();
	var text = $.child(option, true);
	$.reset(option);
	var option_value = {};
	$.template_effect(($0, $1, $2) => {
		$.set_class(option, 1, $0);
		$.set_text(text, $1);
		if (option_value !== (option_value = $2)) {
			option.__value = $2;
		}
	}, void 0, [
		async () => $.clsx((await $.track_reactivity_loss($$props.c))()),
		async () => (await $.track_reactivity_loss($$props.a))(),
		async () => (await $.track_reactivity_loss($$props.a))()
	]);
	$.append($$anchor, option);
	return $.pop($$exports);
}
