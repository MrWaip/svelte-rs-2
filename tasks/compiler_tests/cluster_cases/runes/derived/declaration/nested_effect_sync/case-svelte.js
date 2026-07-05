import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>Click</button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let count = $.state(0);
	$.user_effect(() => {
		let double = $.derived(() => $.get(count) * 2);
		console.log($.get(double));
	});
	var button = root();
	$.delegated("click", button, () => $.update(count));
	$.append($$anchor, button);
	$.pop();
}
$.delegate(["click"]);
