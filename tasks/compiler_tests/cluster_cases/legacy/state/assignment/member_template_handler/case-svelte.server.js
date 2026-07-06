import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = 0;
	$$renderer.push(`<button>obj</button> <button>arr</button> <p>x: ${$.escape(x)}</p>`);
}
