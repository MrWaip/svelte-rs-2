import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let value = "a";
	function onchange() {}
	$$renderer.select({
		id: "s",
		value,
		onchange,
		onfocus: () => value = "b",
		class: "x"
	}, ($$renderer) => {
		$$renderer.option({ value: "a" }, ($$renderer) => {
			$$renderer.push(`a`);
		});
	});
}
