App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let n = $$props["n"];
		function run() {
			try {
				const a = 1;
				const b = 2;
				console.log(a, b);
			} catch {}
		}
		if (n) {
			console.log(n);
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 17, 0);
		$$renderer.push(`${$.escape(n)}</button>`);
		$.pop_element();
		$.bind_props($$props, { n });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
