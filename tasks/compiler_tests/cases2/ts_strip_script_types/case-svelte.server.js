import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let name = "world";
	let status = "active";
	function greet(user) {
		return `Hello ${user.name}`;
	}
	$$renderer.push(`<p>world</p> <p>active</p>`);
}
