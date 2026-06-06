import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
const s = ($$anchor, $$arg0) => {
	let ab = () => $$arg0?.()["a-b"];
	let cd = () => $$arg0?.()["c d"];
	var button = root_1();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${ab() ?? ""}${cd() ?? ""}`));
	$.append($$anchor, button);
};
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let v = {
		"a-b": 1,
		"c d": 2
	};
	s($$anchor, () => v);
}
