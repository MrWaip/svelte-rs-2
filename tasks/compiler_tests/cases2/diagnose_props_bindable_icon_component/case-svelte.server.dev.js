App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { value = "", placeholder = "", type = "text", disabled = false, icon: Icon } = $$props;
		$$renderer.push(`<div class="ui-input-wrapper svelte-nbptzh">`);
		$.push_element($$renderer, "div", 20, 0);
		if (Icon) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<div class="ui-input-icon svelte-nbptzh">`);
			$.push_element($$renderer, "div", 22, 2);
			if (Icon) {
				$$renderer.push("<!--[-->");
				Icon($$renderer, {});
				$$renderer.push("<!--]-->");
			} else {
				$$renderer.push("<!--[!-->");
				$$renderer.push("<!--]-->");
			}
			$$renderer.push(`</div>`);
			$.pop_element();
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]--> <input${$.attr("value", value)}${$.attr("placeholder", placeholder)}${$.attr("type", type)}${$.attr("disabled", disabled, true)} class="ui-input svelte-nbptzh"/>`);
		$.push_element($$renderer, "input", 26, 1);
		$.pop_element();
		$$renderer.push(`</div>`);
		$.pop_element();
		$.bind_props($$props, { value });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
