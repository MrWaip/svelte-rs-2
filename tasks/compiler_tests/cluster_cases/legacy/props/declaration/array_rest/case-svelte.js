import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let tmp = [
		1,
		2,
		3
	], $$array = $.derived(() => $.to_array(tmp)), a = $.prop($$props, "a", 24, () => $.get($$array)[0]), rest = $.prop($$props, "rest", 24, () => $.get($$array).slice(1));
	$.init();
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a() ?? ""}${($.deep_read_state(rest()), $.untrack(() => rest().length)) ?? ""}`));
	$.append($$anchor, button);
	$.pop();
}
