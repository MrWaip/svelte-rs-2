import * as $ from "svelte/internal/server";
import { setContext } from "svelte";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let a = $$props["a"];
		let local = 0;
		const getLocal = () => local;
		setContext("k", a);
		local = a;
		$.bind_props($$props, {
			a,
			getLocal
		});
	});
}
