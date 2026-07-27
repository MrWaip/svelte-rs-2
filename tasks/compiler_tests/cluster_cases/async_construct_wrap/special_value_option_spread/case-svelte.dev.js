import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<option> </option>`), App[$.FILENAME], [[2, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var option = root();
	$.attribute_effect(option, () => ({ ...$$props.rest }));
	var text = $.child(option, true);
	$.reset(option);
	$.template_effect(($0) => $.set_text(text, $0), void 0, [async () => (await $.track_reactivity_loss($$props.a))()]);
	$.append($$anchor, option);
	return $.pop($$exports);
}
