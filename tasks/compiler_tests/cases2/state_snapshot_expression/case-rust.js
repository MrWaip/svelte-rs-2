import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let obj = $.proxy({ a: 1 });
	console.log($.snapshot(obj));
}
