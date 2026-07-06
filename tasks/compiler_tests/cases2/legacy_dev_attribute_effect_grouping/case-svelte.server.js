import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let title = "t";
	let counter = 0;
	let flag = "x";
	function bump() {
		title = title + "!";
		counter += 1;
		flag = flag + "!";
	}
	$$renderer.push(`<button>bump</button> <div${$.attr("title", title)}${$.attr("data-counter", counter)}${$.attr("data-flag", flag)}></div>`);
}
