import * as $ from "svelte/internal/server";
import Comp from "./Comp.svelte";
import { obj } from "./x.js";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { x } = $$props;
		const y = obj.prop;
		Comp($$renderer, { p: y });
	});
}
