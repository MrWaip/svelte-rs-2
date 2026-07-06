App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<h1>`);
		$.push_element($$renderer, "h1", 1, 0);
		$$renderer.push(`<span>`);
		$.push_element($$renderer, "span", 2, 4);
		$$renderer.push(`Entities ${$.escape(logged_in)}</span>`);
		$.pop_element();
		$$renderer.push(` <button>`);
		$.push_element($$renderer, "button", 3, 4);
		$$renderer.push(`+</button>`);
		$.pop_element();
		$$renderer.push(` some long text</h1>`);
		$.pop_element();
		$$renderer.push(` <noscript>`);
		$.push_element($$renderer, "noscript", 8, 0);
		$$renderer.push(`any content</noscript>`);
		$.pop_element();
		$$renderer.push(` `);
		if (!loading) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<div>`);
			$.push_element($$renderer, "div", 11, 4);
			if (featureA) {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<div>`);
				$.push_element($$renderer, "div", 13, 12);
				$$renderer.push(`<div>`);
				$.push_element($$renderer, "div", 14, 16);
				$$renderer.push(`${$.escape(user_name)}</div>`);
				$.pop_element();
				$$renderer.push(` <button>`);
				$.push_element($$renderer, "button", 18, 16);
				$$renderer.push(`${$.escape(counter)}</button>`);
				$.pop_element();
				$$renderer.push(`</div>`);
				$.pop_element();
			} else if (featureB) {
				$$renderer.push("<!--[1-->");
				$$renderer.push(`<div>`);
				$.push_element($$renderer, "div", 21, 12);
				$$renderer.push(`<p>`);
				$.push_element($$renderer, "p", 22, 16);
				$$renderer.push(`Lorem</p>`);
				$.pop_element();
				$$renderer.push(`</div>`);
				$.pop_element();
			} else {
				$$renderer.push("<!--[-1-->");
				$$renderer.push(`<h2>`);
				$.push_element($$renderer, "h2", 25, 12);
				$$renderer.push(`Old UI</h2>`);
				$.pop_element();
			}
			$$renderer.push(`<!--]--></div>`);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.push(`<div>`);
			$.push_element($$renderer, "div", 31, 4);
			$$renderer.push(`Spinner ${$.escape(percent)}</div>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
