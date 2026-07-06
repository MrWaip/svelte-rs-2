import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const a = 100;
	const arr = [{ a: 1 }, 2];
	function go() {
		[arr[0].a, arr[1] = a] = [arr[1]];
	}
	$$renderer.push(`<button>${$.escape(JSON.stringify(arr))}</button>`);
}
