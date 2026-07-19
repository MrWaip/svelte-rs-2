import * as $ from "svelte/internal/client";
const row = ($$anchor, item = $.noop) => {
	const label = item().name;
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, label));
	$.append($$anchor, p);
};
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	row($$anchor, () => ({ name: "x" }));
}
