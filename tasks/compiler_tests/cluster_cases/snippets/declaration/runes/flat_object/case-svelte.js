import * as $ from "svelte/internal/client";
const s = ($$anchor, $$arg0) => {
	let a = () => $$arg0?.().a;
	let b = () => $$arg0?.().b;
	var button = root_1();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a() ?? ""}${b() ?? ""}`));
	$.append($$anchor, button);
};
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let v = $.proxy({
		a: 1,
		b: 2
	});
	s($$anchor, () => v);
}
