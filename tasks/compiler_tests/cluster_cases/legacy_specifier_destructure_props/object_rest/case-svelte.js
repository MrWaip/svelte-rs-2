import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	let tmp = {
		a: 1,
		b: 2,
		c: 3
	}, a = $.prop($$props, "a", 28, () => tmp.a), rest = $.exclude_from_object(tmp, ["a"]);
	function inc() {
		$.update_prop(a);
	}
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(($0) => $.set_text(text, `${a() ?? ""}${$0 ?? ""}`), [() => $.untrack(() => JSON.stringify(rest))]);
	$.event("click", button, inc);
	$.append($$anchor, button);
}
