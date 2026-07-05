import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let size = 1;
	let arr = [];
	function joinClasses(a) {
		return a.join(" ");
	}
	$$renderer.push(`<div${$.attributes({
		...{ id: "x" },
		class: `size_1 ${$.stringify(joinClasses(arr))}`
	})}></div>`);
}
