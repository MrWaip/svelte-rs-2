App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { a, b } = $$props;
		$$renderer.push(`<table>`);
		$.push_element($$renderer, "table", 5, 0);
		$$renderer.push(`<tbody>`);
		$.push_element($$renderer, "tbody", 6, 4);
		$$renderer.push(`<tr>`);
		$.push_element($$renderer, "tr", 7, 8);
		$$renderer.push(`<td>`);
		$.push_element($$renderer, "td", 8, 12);
		if (a) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`${$.escape(a)} <br/>`);
			$.push_element($$renderer, "br", 10, 24);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]--> `);
		if (b) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`${$.escape(b)} <br/>`);
			$.push_element($$renderer, "br", 13, 24);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]--></td>`);
		$.pop_element();
		$$renderer.push(`</tr>`);
		$.pop_element();
		$$renderer.push(`</tbody>`);
		$.pop_element();
		$$renderer.push(`</table>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
