import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<!--[-->`);
		{
			let number;
			$.prevent_snippet_stringification(greet);
			function greet($$renderer) {
				$.validate_snippet_args($$renderer);
				let greeting;
				var promises_1 = $$renderer.run([async () => greeting = (await $.save("hi"))()]);
				$$renderer.async_block([promises[0], promises_1[0]], ($$renderer) => {
					if (number > 4 && greeting) {
						$$renderer.push("<!--[0-->");
						$$renderer.push(`<p>`);
						$.push_element($$renderer, "p", 6, 3);
						$$renderer.push(`yes</p>`);
						$.pop_element();
					} else {
						$$renderer.push("<!--[-1-->");
					}
				});
				$$renderer.push(`<!--]-->`);
			}
			var promises = $$renderer.run([async () => number = (await $.save(Promise.resolve(5)))()]);
			greet($$renderer);
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
