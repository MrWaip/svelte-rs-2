import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let count = 0;
	$.next();
	var text = $.text("hello");
	$.template_effect(() => {
		console.log({ count: $.snapshot(count) });
		debugger;
	});
	$.append($$anchor, text);
}
