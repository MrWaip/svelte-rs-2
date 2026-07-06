import * as $ from "svelte/internal/server";
import Outer from "./Outer.svelte";
import Img from "./Img.svelte";
import Btn from "./Btn.svelte";
export default function App($$renderer) {
	let cond = true;
	Outer($$renderer, { $$slots: {
		image: ($$renderer) => {
			Img($$renderer, { slot: "image" });
		},
		action: ($$renderer) => {
			Btn($$renderer, {
				slot: "action",
				children: ($$renderer) => {
					if (cond) {
						$$renderer.push("<!--[0-->");
						$$renderer.push(`<span>a</span>`);
					} else {
						$$renderer.push("<!--[-1-->");
						$$renderer.push(`b`);
					}
					$$renderer.push(`<!--]-->`);
				},
				$$slots: { default: true }
			});
		}
	} });
}
