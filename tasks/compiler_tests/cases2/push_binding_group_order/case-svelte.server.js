import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { value = "" } = $$props;
		let group = [];
		$$renderer.push(`<input type="radio"${$.attr("checked", group === "a", true)} value="a"/> <input type="radio"${$.attr("checked", group === "b", true)} value="b"/> <p>${$.escape(value)}</p>`);
		$.bind_props($$props, { value });
	});
}
