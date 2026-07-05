import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const PI = 3.14;
	function greet(name) {
		return "Hello " + name;
	}
	$$renderer.push(`<p>PI is 3.14</p>`);
	$.bind_props($$props, {
		PI,
		greet
	});
}
