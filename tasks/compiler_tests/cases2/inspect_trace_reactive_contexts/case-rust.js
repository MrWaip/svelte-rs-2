import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let count = 0;
	let doubled = $.derived(() => {
		return count * 2;
	});
	$.user_effect(() => {
		$.get(doubled);
	});
	$.pop();
}
