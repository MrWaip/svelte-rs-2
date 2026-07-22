import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = 1;
	$$renderer.push(`<img src="a" alt="" loading="lazy"/> <div>1<img src="b" alt="" loading="lazy"/></div>`);
}
