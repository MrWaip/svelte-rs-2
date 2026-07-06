import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let data, details;
	const default_details = { country: "" };
	$: data = {
		locked: false,
		details: null
	};
	$: details = data.details ?? default_details;
	$$renderer.select({
		value: details.country,
		disabled: data.locked
	}, ($$renderer) => {
		$$renderer.option({ value: "" }, ($$renderer) => {
			$$renderer.push(`Select`);
		});
		$$renderer.option({ value: "us" }, ($$renderer) => {
			$$renderer.push(`US`);
		});
	});
}
