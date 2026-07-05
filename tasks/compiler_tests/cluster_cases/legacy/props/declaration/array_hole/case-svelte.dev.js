import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let tmp = [
		1,
		2,
		3
	], $$array = $.derived(() => $.to_array(tmp, 3)), a = $.prop($$props, "a", 24, () => $.get($$array)[0]), c = $.prop($$props, "c", 24, () => $.get($$array)[2]);
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a() ?? ""}${c() ?? ""}`));
	$.append($$anchor, button);
	return $.pop($$exports);
}
