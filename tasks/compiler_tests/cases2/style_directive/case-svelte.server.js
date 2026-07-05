import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let color = "red";
	let fontSize = "16px";
	let bg = "blue";
	let columns = 3;
	const staticVal = "bold";
	$$renderer.push(`<div${$.attr_style("", {
		color,
		"--columns": columns,
		"font-size": fontSize,
		"background-color": bg,
		"font-weight": staticVal
	})}>Styled</div>`);
}
