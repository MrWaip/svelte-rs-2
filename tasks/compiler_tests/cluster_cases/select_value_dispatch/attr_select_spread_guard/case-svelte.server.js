import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { props } = $$props;
	$$renderer.select({
		value: "dog",
		...props
	}, ($$renderer) => {
		$$renderer.option({ value: "dog" }, ($$renderer) => {
			$$renderer.push(`Dog`);
		});
	});
}
