import * as $ from "svelte/internal/client";
const s = ($$anchor, x = $.noop) => {
	var button = root_1();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, x()));
	$.append($$anchor, button);
};
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let v = 1;
	s($$anchor, () => v);
}
