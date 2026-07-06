import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let obj = { a: 1 };
	console.log($.snapshot(obj));
}
