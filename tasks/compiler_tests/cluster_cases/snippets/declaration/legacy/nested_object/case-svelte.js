import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
const s = ($$anchor, $$arg0) => {
	let a = () => $$arg0?.().p.a;
	let b = () => $$arg0?.().q.b;
	var button = root_1();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a() ?? ""}${b() ?? ""}`));
	$.append($$anchor, button);
};
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let v = {
		p: { a: 1 },
		q: { b: 2 }
	};
	s($$anchor, () => v);
}
