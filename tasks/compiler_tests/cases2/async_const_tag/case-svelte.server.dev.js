import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var data;
		var $$promises = $$renderer.run([async () => data = await fetch("/api")]);
		if (true) {
			$$renderer.push("<!--[0-->");
			let value;
			var promises = $$renderer.run([() => $$promises[0], () => value = data.text]);
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 7, 1);
			$$renderer.async([promises[1]], ($$renderer) => $$renderer.push(() => $.escape(value)));
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
