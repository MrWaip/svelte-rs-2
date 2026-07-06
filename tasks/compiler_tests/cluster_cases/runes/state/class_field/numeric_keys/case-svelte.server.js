import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Test {
			0;
			1;
		}
		const test = new Test();
	});
}
