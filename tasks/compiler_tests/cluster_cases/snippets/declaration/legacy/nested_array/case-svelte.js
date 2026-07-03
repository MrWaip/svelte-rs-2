import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
const s = ($$anchor, $$arg0) => {
	var $$array = $.derived(() => $.to_array($$arg0?.(), 2));
	var $$array_1 = $.derived(() => $.to_array($$array[0], 2));
	var $$array_2 = $.derived(() => $.to_array($$array[1], 2));
	let a = () => $.get($$array_1)[0];
	let b = () => $.get($$array_1)[1];
	let c = () => $.get($$array_2)[0];
	let d = () => $.get($$array_2)[1];
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a() ?? ""}${b() ?? ""}${c() ?? ""}${d() ?? ""}`));
	$.append($$anchor, button);
};
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let v = [[1, 2], [3, 4]];
	s($$anchor, () => v);
}
