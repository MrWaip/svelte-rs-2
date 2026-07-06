App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let href = $.fallback($$props["href"], undefined);
		function getTag() {
			return href ? "a" : "div";
		}
		function getRole() {
			return href ? "link" : undefined;
		}
		const $$tag = getTag();
		$.validate_dynamic_element_tag(() => $$tag);
		$.validate_void_dynamic_element(() => $$tag);
		$.push_element($$renderer, $$tag, 14, 0);
		$.element($$renderer, $$tag, () => {
			$$renderer.push(`${$.attr("role", getRole())}${$.attr("href", href)}`);
		}, () => {
			$$renderer.push(`x`);
		});
		$.pop_element();
		$.bind_props($$props, { href });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
