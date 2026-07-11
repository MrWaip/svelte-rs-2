App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { confirmStore } = $$props;
		const $$d = $.derived(() => confirmStore.data), phone = $.derived(() => $$d().phone), rate = $.derived(() => $$d().rate);
		$$renderer.push(`<span>`);
		$.push_element($$renderer, "span", 5, 0);
		$$renderer.push(`${$.escape(phone())}${$.escape(rate())}</span>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
