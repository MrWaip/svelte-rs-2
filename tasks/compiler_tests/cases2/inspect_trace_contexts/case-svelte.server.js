import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	let data = null;
	const handleArrow = () => {
		count++;
	};
	async function fetchData() {
		data = await fetch("/api");
	}
	foo(() => {
		count++;
	});
	const obj = { handler() {
		count++;
	} };
	$$renderer.push(`<button>Click</button>`);
}
