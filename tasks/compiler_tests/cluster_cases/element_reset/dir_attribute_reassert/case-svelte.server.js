import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = 1;
	$$renderer.push(`<p dir="rtl">text</p> <div>1<p dir="auto">dynamic parent reset</p></div>`);
}
