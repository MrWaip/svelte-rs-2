import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
const s = ($$anchor, $$arg0) => {
	let a = () => ($$arg0?.()).a;
	let rest = () => $.exclude_from_object($$arg0?.(), ["a"]);
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a() ?? ""}${(rest(), $.untrack(() => rest().b)) ?? ""}`));
	$.append($$anchor, button);
};
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let v = {
		a: 1,
		b: 2,
		c: 3
	};
	s($$anchor, () => v);
}
