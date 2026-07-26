import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function delay(value) {
		return Promise.resolve(value);
	}
	var attrs;
	var $$promises = $$renderer.run([async () => attrs = await delay({ title: "hi" })]);
	$$renderer.async([$$promises[0]], ($$renderer) => {
		$$renderer.push(`<div${$.attributes({ ...attrs })}></div>`);
	});
}
