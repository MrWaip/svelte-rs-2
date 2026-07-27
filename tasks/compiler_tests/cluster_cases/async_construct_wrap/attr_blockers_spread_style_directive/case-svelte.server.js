import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { rest } = $$props;
	var color;
	var $$promises = $$renderer.run([() => Promise.resolve(), () => color = "red"]);
	$$renderer.async([$$promises[1]], ($$renderer) => {
		$$renderer.push(`<div${$.attributes({ ...rest }, void 0, void 0, { color })}></div>`);
	});
}
