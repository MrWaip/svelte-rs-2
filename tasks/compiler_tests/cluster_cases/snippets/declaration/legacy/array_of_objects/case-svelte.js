import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
const s = ($$anchor, $$arg0) => {
	var $$array = $.derived(() => $.to_array($$arg0?.(), 2));
	let a = () => $.get($$array)[0].a;
	let b = () => $.get($$array)[1].b;
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a() ?? ""}${b() ?? ""}`));
	$.append($$anchor, button);
};
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let v = [{ a: 1 }, { b: 2 }];
	s($$anchor, () => v);
}
