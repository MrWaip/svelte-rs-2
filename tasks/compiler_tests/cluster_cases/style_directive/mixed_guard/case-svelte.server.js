import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let a = "red";
	let b = "blue";
	$$renderer.push(`<button>go</button> <div${$.attr_style("", {
		color: a,
		background: b
	})}></div>`);
}
