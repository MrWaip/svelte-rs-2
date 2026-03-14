import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let count = $.state(0);
	$.set(count, $.get(count) + 1);
	$.user_effect(() => {
		console.log("count:", $.get(count));
	});
	$.user_pre_effect(() => {
		console.log("pre-effect:", $.get(count));
	});
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(count)));
	$.append($$anchor, p);
	$.pop();
}
