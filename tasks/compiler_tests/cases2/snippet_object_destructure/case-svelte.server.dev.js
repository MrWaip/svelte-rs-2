App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
$.prevent_snippet_stringification(greeting);
function greeting($$renderer, { name, age }) {
	$.validate_snippet_args($$renderer);
	$$renderer.push(`<p>`);
	$.push_element($$renderer, "p", 6, 1);
	$$renderer.push(`${$.escape(name)} is ${$.escape(age)}</p>`);
	$.pop_element();
}
$.prevent_snippet_stringification(withDefault);
function withDefault($$renderer, { label = "default" }) {
	$.validate_snippet_args($$renderer);
	$$renderer.push(`<span>`);
	$.push_element($$renderer, "span", 10, 1);
	$$renderer.push(`${$.escape(label)}</span>`);
	$.pop_element();
}
$.prevent_snippet_stringification(withRest);
function withRest($$renderer, { id, ...rest }) {
	$.validate_snippet_args($$renderer);
	$$renderer.push(`<div>`);
	$.push_element($$renderer, "div", 14, 1);
	$$renderer.push(`${$.escape(id)}</div>`);
	$.pop_element();
}
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let data = {
			name: "world",
			age: 25
		};
		greeting($$renderer, data);
		$$renderer.push(`<!----> `);
		withDefault($$renderer, {});
		$$renderer.push(`<!----> `);
		withRest($$renderer, {
			id: 1,
			extra: true
		});
		$$renderer.push(`<!---->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
