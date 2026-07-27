App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let a = 0;
		const handler = $.derived(() => a ? () => {
			a++;
		} : undefined);
		const list = $.derived(() => [() => a, () => a + 1]);
		const cfg = $.derived(() => ({ run: () => a }));
		const call = $.derived(() => [1].map(() => a));
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 9, 0);
		$$renderer.push(`${$.escape(a)} ${$.escape(list().length)} ${$.escape(cfg().run())} ${$.escape(call().length)}</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
