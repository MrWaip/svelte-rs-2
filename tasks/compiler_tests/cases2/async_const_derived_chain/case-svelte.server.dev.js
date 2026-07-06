import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var data;
		var $$promises = $$renderer.run([async () => data = await fetch("/api")]);
		if (true) {
			$$renderer.push("<!--[0-->");
			let a;
			let b;
			let c;
			var promises = $$renderer.run([
				() => $$promises[0],
				() => a = data.value,
				() => b = a * 2,
				() => c = b + 1
			]);
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 9, 1);
			$$renderer.async([promises[3]], ($$renderer) => $$renderer.push(() => $.escape(c)));
			$$renderer.push(`</p>`);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
