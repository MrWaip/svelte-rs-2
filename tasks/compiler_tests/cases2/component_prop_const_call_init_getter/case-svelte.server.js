import * as $ from "svelte/internal/server";
import Comp from "./Comp.svelte";
import { foo } from "./x.js";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { x } = $$props;
		const editUrl = foo();
		Comp($$renderer, {
			href: editUrl,
			radius: 16
		});
	});
}
