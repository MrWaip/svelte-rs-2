App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let obj = {
			items: null,
			data: null,
			list: null,
			map: null
		};
		// Non-statement assignment — should use $.assign_nullish in dev
		(obj.items ??= []).push(1);
		// Non-statement assignment — should use $.assign in dev
		(obj.data = []).push(2);
		// Non-statement — $.assign_and
		(obj.list &&= []).length;
		// Non-statement — $.assign_or
		(obj.map ||= []).length;
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 17, 0);
		$$renderer.push(`${$.escape(obj.items)}</p>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
