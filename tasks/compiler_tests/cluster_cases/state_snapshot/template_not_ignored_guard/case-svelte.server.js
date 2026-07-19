import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let arr = { test: 1 };
	$$renderer.push(`<div${$.attributes({ ...$.snapshot(arr) })}>a</div>`);
}
