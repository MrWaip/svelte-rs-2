import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
$.prevent_snippet_stringification(row);
function row($$renderer, value) {
	$.validate_snippet_args($$renderer);
	$$renderer.push(`<p>`);
	$.push_element($$renderer, "p", 9, 21);
	$$renderer.push(`${$.escape(value)}</p>`);
	$.pop_element();
}
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let x = 0;
		function delay(value) {
			return Promise.resolve({ value });
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 11, 0);
		$$renderer.push(`inc</button>`);
		$.pop_element();
		$$renderer.push(` `);
		$$renderer.child_block(async ($$renderer) => {
			const $$0 = (await $.save((await $.save(delay(x)))().value))();
			row($$renderer, $$0);
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
