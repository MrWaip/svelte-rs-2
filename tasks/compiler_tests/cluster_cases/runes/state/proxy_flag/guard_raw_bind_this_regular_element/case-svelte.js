import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let elem = $.state(void 0);
	$.user_effect(() => {
		console.log($.get(elem));
	});
	var div = root();
	$.bind_this(div, ($$value) => $.set(elem, $$value), () => $.get(elem));
	$.append($$anchor, div);
	$.pop();
}
