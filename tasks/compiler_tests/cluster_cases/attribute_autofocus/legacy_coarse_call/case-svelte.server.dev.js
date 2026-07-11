App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let adapter = $$props["adapter"];
		let day = $$props["day"];
		let focused = null;
		function pick() {
			focused = day;
		}
		$$renderer.push(`<button${$.attr("autofocus", focused !== null && adapter.isSame(day, focused), true)}>`);
		$.push_element($$renderer, "button", 12, 0);
		$$renderer.push(`</button>`);
		$.pop_element();
		$.bind_props($$props, {
			adapter,
			day
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
