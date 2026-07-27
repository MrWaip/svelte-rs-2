App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
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
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 9, 0);
		$$renderer.push(`${$.escape(plain())} ${$.escape(named())} ${$.escape(typeof asyncf())} ${$.escape(memb())}</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
