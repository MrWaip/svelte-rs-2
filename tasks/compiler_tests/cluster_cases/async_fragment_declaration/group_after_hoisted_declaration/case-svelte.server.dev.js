import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const id = "name";
		{
			const nested = "nested";
			let greeting2;
			var promises = $$renderer.run([async () => greeting2 = await $.async_derived(async () => (await $.save(`Hi ${id}`))())]);
			$$renderer.push(`<div>`);
			$.push_element($$renderer, "div", 2, 0);
			$$renderer.push(`<span>`);
			$.push_element($$renderer, "span", 5, 1);
			$$renderer.push(`nested `);
			$$renderer.async([promises[0]], ($$renderer) => $$renderer.push(() => $.escape(greeting2())));
			$$renderer.push(`</span>`);
			$.pop_element();
			$$renderer.push(`</div>`);
			$.pop_element();
		}
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
