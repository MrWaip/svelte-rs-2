import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = 1;
	let y = 2;
	function inc() {
		x++;
		y++;
	}
	$$renderer.push(`<button>${$.escape(x)}${$.escape(y)}</button>`);
}
