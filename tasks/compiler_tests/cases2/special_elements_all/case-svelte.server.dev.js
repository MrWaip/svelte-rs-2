App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let count = 0;
		function handleEvent() {
			count++;
		}
		function action(node) {
			return { destroy() {} };
		}
		$.head("q2w0q4", $$renderer, ($$renderer) => {
			$$renderer.title(($$renderer) => {
				$$renderer.push(`<title>App: ${$.escape(count)}</title>`);
			});
			$$renderer.push(`<meta name="viewport" content="width=device-width"/>`);
			$.push_element($$renderer, "meta", 15, 4);
			$.pop_element();
		});
		$$renderer.push(`<div>`);
		$.push_element($$renderer, "div", 22, 0);
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 23, 4);
		$$renderer.push(`Count: ${$.escape(count)}</p>`);
		$.pop_element();
		$$renderer.push(`</div>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
