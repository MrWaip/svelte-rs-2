import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { value = "", placeholder = "", type = "text", disabled = false, icon: Icon } = $$props;
		$$renderer.push(`<div class="ui-input-wrapper svelte-nbptzh">`);
		if (Icon) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<div class="ui-input-icon svelte-nbptzh">`);
			if (Icon) {
				$$renderer.push("<!--[-->");
				Icon($$renderer, {});
				$$renderer.push("<!--]-->");
			} else {
				$$renderer.push("<!--[!-->");
				$$renderer.push("<!--]-->");
			}
			$$renderer.push(`</div>`);
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]--> <input${$.attr("value", value)}${$.attr("placeholder", placeholder)}${$.attr("type", type)}${$.attr("disabled", disabled, true)} class="ui-input svelte-nbptzh"/></div>`);
		$.bind_props($$props, { value });
	});
}
