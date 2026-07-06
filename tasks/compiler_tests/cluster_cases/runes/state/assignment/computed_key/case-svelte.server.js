import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let a = 0;
	let key = "prop";
	let obj = { prop: 1 };
	function update() {
		({[key]: a} = obj);
	}
	$$renderer.push(`<button>${$.escape(a)}</button>`);
}
