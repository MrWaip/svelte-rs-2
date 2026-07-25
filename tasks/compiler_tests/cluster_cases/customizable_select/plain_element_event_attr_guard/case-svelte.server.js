import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let rest = {};
	function onclick() {}
	$$renderer.push(`<div${$.attributes({
		...rest,
		class: "x"
	})}></div> <button class="y">b</button>`);
}
