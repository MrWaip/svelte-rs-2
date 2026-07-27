import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let count = 0;
		const promise = Promise.resolve(1);
		if (count >= 0) {
			$$renderer.push("<!--[0-->");
			let value;
			var promises = $$renderer.run([async () => value = (await $.save(promise))()]);
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 8, 1);
			$$renderer.async([promises[0]], ($$renderer) => $$renderer.push(() => $.escape(value)));
			$$renderer.push(` ${$.escape(count)}</p>`);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]--> <button>`);
		$.push_element($$renderer, "button", 10, 0);
		$$renderer.push(`inc</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
