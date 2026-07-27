App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
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
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 12, 0);
		$$renderer.push(`${$.escape(g5())} ${$.escape(g6())} ${$.escape(g7())} ${$.escape(typeof g1())} ${$.escape(typeof g2())} ${$.escape(g3()().x)} ${$.escape(g4()())}</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
