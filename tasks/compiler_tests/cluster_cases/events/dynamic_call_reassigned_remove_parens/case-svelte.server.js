import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let makeHandler = null;
	makeHandler = () => () => console.log("x");
	$$renderer.push(`<button>x</button>`);
}
