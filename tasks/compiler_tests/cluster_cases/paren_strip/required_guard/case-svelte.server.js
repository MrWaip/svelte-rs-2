import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let a = 0;
		const g1 = $.derived(() => a && (() => a));
		const g2 = $.derived(() => a ?? (() => a));
		const g3 = $.derived(() => () => ({ x: a }));
		const g4 = $.derived(() => (a, () => a));
		const g5 = $.derived(() => (a + 1) * 2);
		const g6 = $.derived(() => (() => a)());
		const g7 = $.derived(() => (function() {
			return a;
		})());
		$$renderer.push(`<button>${$.escape(g5())} ${$.escape(g6())} ${$.escape(g7())} ${$.escape(typeof g1())} ${$.escape(typeof g2())} ${$.escape(g3()().x)} ${$.escape(g4()())}</button>`);
	});
}
