import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let props = {
		id: "a",
		style: "border-color: blue;"
	};
	let color = "red";
	$$renderer.push(`<div${$.attributes({ ...props }, void 0, void 0, { color })}></div>`);
}
