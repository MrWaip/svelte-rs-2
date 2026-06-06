import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let tmp = {
		a: 1,
		b: 2,
		c: 3
	}, a = $.prop($$props, "a", 24, () => tmp.a), rest = $.prop($$props, "rest", 24, () => $.exclude_from_object(tmp, ["a"]));
	$.init();
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a() ?? ""}${($.deep_read_state(rest()), $.untrack(() => rest().b)) ?? ""}`));
	$.append($$anchor, button);
	$.pop();
}
