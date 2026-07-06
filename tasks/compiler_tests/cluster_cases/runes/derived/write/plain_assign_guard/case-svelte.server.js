import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let s = 0;
	let d = $.derived(() => s * 2);
	$$renderer.push(`<button>x</button> <button>s</button> ${$.escape(d())}`);
}
