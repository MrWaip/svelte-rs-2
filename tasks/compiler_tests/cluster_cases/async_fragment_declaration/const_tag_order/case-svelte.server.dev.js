import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let n = 1;
		if (n) {
			$$renderer.push("<!--[0-->");
			const before = "sync";
			let awaited;
			let after;
			var promises = $$renderer.run([async () => awaited = (await $.save(Promise.resolve(n)))(), () => after = awaited + 1]);
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 9, 1);
			$$renderer.push(`sync`);
			$$renderer.async([promises[0]], ($$renderer) => $$renderer.push(() => $.escape(awaited)));
			$$renderer.async([promises[1]], ($$renderer) => $$renderer.push(() => $.escape(after)));
			$$renderer.push(`</p>`);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]--> <button>`);
		$.push_element($$renderer, "button", 11, 0);
		$$renderer.push(`go</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
