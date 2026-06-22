import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
const s = ($$anchor, $$arg0) => {
	var $$array = $.derived(() => $.to_array(($$arg0?.()).outer, 1));
	let inner = () => $.get($$array)[0].inner;
	var button = root_1();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, inner()));
	$.append($$anchor, button);
};
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let v = { outer: [{ inner: 1 }] };
	s($$anchor, () => v);
}
