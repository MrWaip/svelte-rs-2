import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let count = $.state(0);
	$.user_effect(() => {
		const interval = setInterval(() => {
			$.set(count, $.get(count) + 1);
		}, 1e3);
		return () => {
			clearInterval(interval);
		};
	});
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(count)));
	$.append($$anchor, p);
	$.pop();
}
