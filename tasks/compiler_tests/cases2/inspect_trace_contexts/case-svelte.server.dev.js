App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let count = 0;
		let data = null;
		const handleArrow = () => {
			count++;
		};
		async function fetchData() {
			data = await fetch("/api");
		}
		foo(() => {
			count++;
		});
		const obj = { handler() {
			count++;
		} };
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 28, 0);
		$$renderer.push(`Click</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
