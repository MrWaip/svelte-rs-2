import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { getData } = $$props;
		const value = $.derived(getData);
		$$renderer.push(`<span>${$.escape(value())}</span>`);
	});
}
