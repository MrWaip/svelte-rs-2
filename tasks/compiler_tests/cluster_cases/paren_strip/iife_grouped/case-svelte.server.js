import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let a = 0;
		const plain = $.derived(() => (function() {
			return a;
		})(a));
		const named = $.derived(() => (function f() {
			return a;
		})(a));
		const asyncf = $.derived(() => (async function() {
			return a;
		})(a));
		const memb = $.derived(() => (function() {
			return { x: a };
		})(a).x);
		$$renderer.push(`<button>${$.escape(plain())} ${$.escape(named())} ${$.escape(typeof asyncf())} ${$.escape(memb())}</button>`);
	});
}
