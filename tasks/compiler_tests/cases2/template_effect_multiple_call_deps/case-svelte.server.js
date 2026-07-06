import * as $ from "svelte/internal/server";
import { scale } from "./utils.js";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let data = [
			10,
			20,
			30
		];
		const x = $.derived(() => scale([0, data.length], [0, 100]));
		const y = $.derived(() => scale([0, 30], [100, 0]));
		$$renderer.push(`<polyline${$.attr("points", data.map((d, i) => [x()(i), y()(d)]).join(" "))}></polyline>`);
	});
}
