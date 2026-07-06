import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let value = 0;
	const handler = () => {
		const local = 5;
		value = local;
	};
	$$renderer.push(`<button>${$.escape(value)}</button>`);
}
