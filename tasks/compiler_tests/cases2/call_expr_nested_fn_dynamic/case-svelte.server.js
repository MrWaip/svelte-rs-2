import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let value = 42;
	const double = (n) => n * 2;
	const get_value = () => value;
	$$renderer.push(`<p>${$.escape(double(get_value()))}</p>`);
}
