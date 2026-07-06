import * as $ from "svelte/internal/server";
import { realValue } from "./utils";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let data = { value: 0 };
		function process(input) {
			return realValue.transform(input);
		}
		$$renderer.push(`<button>${$.escape(realValue.label)}</button>`);
	});
}
