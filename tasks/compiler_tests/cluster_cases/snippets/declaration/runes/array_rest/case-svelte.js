import * as $ from "svelte/internal/client";
const s = ($$anchor, $$arg0) => {
	var $$array = $.derived(() => $.to_array($$arg0?.()));
	let a = () => $.get($$array)[0];
	let rest = () => $.get($$array).slice(1);
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a() ?? ""}${rest().length ?? ""}`));
	$.append($$anchor, button);
};
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let v = $.proxy([
		1,
		2,
		3
	]);
	s($$anchor, () => v);
}
