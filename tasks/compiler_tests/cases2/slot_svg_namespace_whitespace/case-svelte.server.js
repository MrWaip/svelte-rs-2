import * as $ from "svelte/internal/server";
import Svg from "./Svg.svelte";
export default function App($$renderer) {
	let paths = [];
	let polygons = [];
	Svg($$renderer, {
		children: ($$renderer) => {
			$$renderer.push(`<!--[-->`);
			const each_array = $.ensure_array_like(paths);
			for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
				let path = each_array[$$index];
				$$renderer.push(`<path${$.attributes({ ...path }, void 0, void 0, void 0, 3)}></path>`);
			}
			$$renderer.push(`<!--]--><!--[-->`);
			const each_array_1 = $.ensure_array_like(polygons);
			for (let $$index_1 = 0, $$length = each_array_1.length; $$index_1 < $$length; $$index_1++) {
				let polygon = each_array_1[$$index_1];
				$$renderer.push(`<polygon${$.attributes({ ...polygon }, void 0, void 0, void 0, 3)}></polygon>`);
			}
			$$renderer.push(`<!--]-->`);
		},
		$$slots: { default: true }
	});
}
