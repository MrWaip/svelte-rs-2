import * as $ from "svelte/internal/server";
import { Foo } from "./x.js";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { value } = $$props;
		if (value === Foo.X) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`a`);
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.push(`b`);
		}
		$$renderer.push(`<!--]-->`);
	});
}
