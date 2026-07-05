import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	/** @type {{ name: string, count?: number }} */
	let { name, count = 0 } = $$props;
	/** @type {number} */
	let doubled = $.derived(() => count * 2);
	/** @type {number} */
	let label = $.derived(() => {
		// format with prefix
		return `${name}: ${doubled()}`;
	});
	$$renderer.push(`<p>${$.escape(label())}</p>`);
}
