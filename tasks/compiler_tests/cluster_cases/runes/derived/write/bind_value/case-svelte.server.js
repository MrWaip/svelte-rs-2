import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let s = 0;
	let d = $.derived(() => s * 2);
	$$renderer.push(`<input${$.attr("value", d())}/> <button>s</button> ${$.escape(d())}`);
}
