import * as $ from "svelte/internal/client";
const greeting = ($$anchor, msg = $.noop) => {
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `Hello ${msg() ?? ""}`));
	$.append($$anchor, p);
};
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	greeting($$anchor, () => "world");
}
