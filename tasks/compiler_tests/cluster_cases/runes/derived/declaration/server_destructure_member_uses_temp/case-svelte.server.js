import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { confirmStore } = $$props;
		const $$d = $.derived(() => confirmStore.data), phone = $.derived(() => $$d().phone), rate = $.derived(() => $$d().rate);
		$$renderer.push(`<span>${$.escape(phone())}${$.escape(rate())}</span>`);
	});
}
