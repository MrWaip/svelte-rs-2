import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	let tmp = [
		1,
		2,
		3
	], $$array = $.derived(() => $.to_array(tmp, 3)), a = $.prop($$props, "a", 24, () => $.get($$array)[0]), c = $.prop($$props, "c", 24, () => $.get($$array)[2]);
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a() ?? ""}${c() ?? ""}`));
	$.append($$anchor, button);
}
