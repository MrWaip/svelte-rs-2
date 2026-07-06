import * as $ from "svelte/internal/server";
import { A } from "./a";
import { B } from "./b";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let y;
		let x = $$props["x"];
		$: y = (function() {
			switch (x) {
				case A.ONE: return B;
				default: return "";
			}
		})();
		$$renderer.push(`<p>${$.escape(y)}</p>`);
		$.bind_props($$props, { x });
	});
}
