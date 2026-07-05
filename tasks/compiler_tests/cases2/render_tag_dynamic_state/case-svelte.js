import * as $ from "svelte/internal/client";
const greeting = ($$anchor, name = $.noop) => {
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `Hello ${name() ?? ""}`));
	$.append($$anchor, p);
};
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let show = null;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.snippet(node, () => show);
	$.append($$anchor, fragment);
}
