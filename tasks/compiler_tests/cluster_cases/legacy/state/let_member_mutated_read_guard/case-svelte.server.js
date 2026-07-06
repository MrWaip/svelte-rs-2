import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let obj = { k: 1 };
	const bump = () => {
		obj.k = 2;
	};
	$$renderer.push(`<button>${$.escape(obj.k)}</button>`);
}
