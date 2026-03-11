import * as $ from "svelte/internal/client";
const greeting = ($$anchor, name = $.noop) => {
	var p = root_1();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `Hello ${name() ?? ""}`));
	$.append($$anchor, p);
};
var root_1 = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let message = "hello";
	greeting($$anchor, () => message);
}
