import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let count = $.state(0);
	var $$exports = {
		get count() {
			return $.get(count);
		},
		set count($$value) {
			$.set(count, $$value);
		}
	};
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(count)));
	$.append($$anchor, p);
	return $.pop($$exports);
}
