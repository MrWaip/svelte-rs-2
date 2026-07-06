import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let numbers = [
		1,
		2,
		3
	];
	function add() {
		numbers.push(numbers.length + 1);
		numbers = numbers;
	}
	$$renderer.push(`<p>${$.escape(numbers.length)}</p> <button>add</button>`);
}
