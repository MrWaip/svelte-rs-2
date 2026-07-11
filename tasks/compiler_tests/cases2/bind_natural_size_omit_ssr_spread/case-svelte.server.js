import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let rest = {};
	let nw = 0;
	let nh = 0;
	$$renderer.push(`<img${$.attributes({
		alt: "",
		...rest
	})} onload="this.__e=event" onerror="this.__e=event"/>`);
}
