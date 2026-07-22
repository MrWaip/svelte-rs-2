import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let a = 0;
		const handler = $.derived(() => a ? () => {
			a++;
		} : undefined);
		const list = $.derived(() => [() => a, () => a + 1]);
		const cfg = $.derived(() => ({ run: () => a }));
		const call = $.derived(() => [1].map(() => a));
		$$renderer.push(`<button>${$.escape(a)} ${$.escape(list().length)} ${$.escape(cfg().run())} ${$.escape(call().length)}</button>`);
	});
}
