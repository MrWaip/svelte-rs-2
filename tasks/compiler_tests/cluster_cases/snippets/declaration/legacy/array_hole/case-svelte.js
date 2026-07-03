import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
const s = ($$anchor, $$arg0) => {
	var $$array = $.derived(() => $.to_array($$arg0?.(), 3));
	let a = () => $.get($$array)[0];
	let c = () => $.get($$array)[2];
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a() ?? ""}${c() ?? ""}`));
	$.append($$anchor, button);
};
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let v = [
		1,
		2,
		3
	];
	s($$anchor, () => v);
}
