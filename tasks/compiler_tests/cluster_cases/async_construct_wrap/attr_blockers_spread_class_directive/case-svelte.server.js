import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { rest } = $$props;
	var a;
	var $$promises = $$renderer.run([() => Promise.resolve(), () => a = "a"]);
	$$renderer.async([$$promises[1]], ($$renderer) => {
		$$renderer.push(`<div${$.attributes({ ...rest }, void 0, { one: a })}></div>`);
	});
}
