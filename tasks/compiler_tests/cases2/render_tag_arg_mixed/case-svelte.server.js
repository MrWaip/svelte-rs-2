import * as $ from "svelte/internal/server";
function show($$renderer, greeting, person) {
	$$renderer.push(`<p>${$.escape(greeting)} ${$.escape(person)}</p>`);
}
export default function App($$renderer) {
	let name = "world";
	function greet() {
		return "hello";
	}
	show($$renderer, greet(), name);
}
