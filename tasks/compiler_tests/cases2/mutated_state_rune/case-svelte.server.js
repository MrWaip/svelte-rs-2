import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let title = 10;
	let flag = void 0;
	let flag2 = void 0;
	let value = "text";
	onMount(() => {
		title = 20;
		window.id = title;
		flag2 = title;
		map(title);
	});
	function map(value, off = title) {
		return value;
	}
	value += 1234;
	value -= 4e3;
	value *= 2;
	value &&= fallback;
	value = "";
	const obj = {
		title,
		title
	};
	$$renderer.push(`<div>${$.escape(title)}</div> <div${$.attr("flag", flag)}>${$.escape(flag2)}</div>`);
}
