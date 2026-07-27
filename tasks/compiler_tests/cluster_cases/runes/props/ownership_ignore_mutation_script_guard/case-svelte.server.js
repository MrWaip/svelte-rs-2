import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { test } = $$props;
		function go() {
			//svelte-ignore ownership_invalid_mutation
			test.test = Math.random();
		}
		$$renderer.push(`<button></button>`);
	});
}
