import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	const promise = fetch("/api");
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => promise, null, void 0, ($$anchor, error) => {
		var p = root();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, $.get(error).message));
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
}
