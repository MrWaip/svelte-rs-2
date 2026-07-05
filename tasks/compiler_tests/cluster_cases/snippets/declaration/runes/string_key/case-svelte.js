import * as $ from "svelte/internal/client";
const s = ($$anchor, $$arg0) => {
	let ab = () => ($$arg0?.())["a-b"];
	let cd = () => ($$arg0?.())["c d"];
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${ab() ?? ""}${cd() ?? ""}`));
	$.append($$anchor, button);
};
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let v = $.proxy({
		"a-b": 1,
		"c d": 2
	});
	s($$anchor, () => v);
}
