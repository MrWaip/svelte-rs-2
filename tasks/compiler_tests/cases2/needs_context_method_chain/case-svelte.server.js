import * as $ from "svelte/internal/server";
import { createFormatter } from "./utils.js";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { data } = $$props;
		let items = [];
		// member on call result: items.filter(Boolean).length
		let total = $.derived(() => items.filter(Boolean).length);
		// call on import: createFormatter()
		let fmt = createFormatter();
		// member on import: createFormatter.defaults
		let defaults = createFormatter.defaults;
		// member on prop: data.nested
		let nested = data.nested;
		// new expression
		let map = new Map();
		$$renderer.push(`<p>${$.escape(total())} ${$.escape(fmt)} ${$.escape(defaults)} ${$.escape(nested)} ${$.escape(map)}</p>`);
	});
}
