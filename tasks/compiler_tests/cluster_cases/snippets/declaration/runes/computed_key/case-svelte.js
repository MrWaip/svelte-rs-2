import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	const s = ($$anchor, $$arg0) => {
		let v = () => ($$arg0?.())[k];
		var button = root_1();
		var text = $.child(button, true);
		$.reset(button);
		$.template_effect(() => $.set_text(text, v()));
		$.append($$anchor, button);
	};
	const k = "z";
	let v = $.proxy({ z: 1 });
	s($$anchor, () => v);
}
