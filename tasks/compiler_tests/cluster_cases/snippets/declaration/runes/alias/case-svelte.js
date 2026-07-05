import * as $ from "svelte/internal/client";
const s = ($$anchor, $$arg0) => {
	let x = () => ($$arg0?.()).a;
	let y = () => ($$arg0?.()).b;
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${x() ?? ""}${y() ?? ""}`));
	$.append($$anchor, button);
};
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let v = $.proxy({
		a: 1,
		b: 2
	});
	s($$anchor, () => v);
}
