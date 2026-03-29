import * as $ from "svelte/internal/client";
var root = $.from_html(`<p></p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let obj = { count: 0 };
	const get_count = () => obj.count;
	$.user_effect(() => {
		obj.count += 1;
	});
	var p = root();
	p.textContent = obj.toString();
	$.append($$anchor, p);
	$.pop();
}
