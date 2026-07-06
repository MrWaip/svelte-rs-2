import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { value = "a" } = $$props;
	$$renderer.select({ value }, ($$renderer) => {
		$$renderer.option({}, ($$renderer) => {
			$$renderer.push(`a`);
		});
		$$renderer.option({}, ($$renderer) => {
			$$renderer.push(`b`);
		});
	});
}
