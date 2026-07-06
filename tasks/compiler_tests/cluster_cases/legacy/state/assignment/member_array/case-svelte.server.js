import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let arr = [1, 2];
	function swap() {
		[arr[0], arr[1]] = [arr[1], arr[0]];
	}
	$$renderer.push(`<button>${$.escape(arr[0])}${$.escape(arr[1])}</button>`);
}
