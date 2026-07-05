App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let handler;
		let onInput = $.fallback($$props["onInput"], () => {});
		let flag = false;
		$: handler = async (value) => {
			const result = await onInput(value, flag);
			if (result) {
				flag = true;
			}
		};
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 16, 0);
		$$renderer.push(`go</button>`);
		$.pop_element();
		$.bind_props($$props, { onInput });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
