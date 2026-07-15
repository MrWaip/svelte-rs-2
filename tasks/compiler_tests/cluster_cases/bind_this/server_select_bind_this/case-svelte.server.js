import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let ref;
	let val = "a";
	$$renderer.select({
		value: val,
		this: ref
	}, ($$renderer) => {
		$$renderer.option({}, ($$renderer) => {
			$$renderer.push(`a`);
		});
	});
}
