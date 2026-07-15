import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { source } = $$props;
	const phone = $.derived(() => source.phone), rate = $.derived(() => source.rate);
	$$renderer.push(`<span>${$.escape(phone())}${$.escape(rate())}</span>`);
}
