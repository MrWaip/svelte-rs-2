import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { rest } = $$props;
	var value;
	var $$promises = $$renderer.run([() => Promise.resolve(), () => value = "value"]);
	$$renderer.async([$$promises[1]], ($$renderer) => {
		$$renderer.push(`<div${$.attributes({
			...rest,
			title: value
		})}></div>`);
	});
}
