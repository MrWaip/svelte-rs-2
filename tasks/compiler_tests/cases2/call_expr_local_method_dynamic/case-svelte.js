import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let obj = { count: 0 };
	const get_count = () => obj.count;
	$.user_effect(() => {
		obj.count += 1;
	});
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(($0) => $.set_text(text, $0), [() => obj.toString()]);
	$.append($$anchor, p);
	$.pop();
}
