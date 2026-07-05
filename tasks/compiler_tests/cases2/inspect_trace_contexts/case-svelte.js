import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>Click</button>`);
export default function App($$anchor) {
	let count = $.state(0);
	let data = $.state(null);
	const handleArrow = () => {
		$.update(count);
	};
	async function fetchData() {
		$.set(data, await fetch("/api"), true);
	}
	foo(() => {
		$.update(count);
	});
	const obj = { handler() {
		$.update(count);
	} };
	var button = root();
	$.delegated("click", button, () => {
		$.update(count);
	});
	$.append($$anchor, button);
}
$.delegate(["click"]);
