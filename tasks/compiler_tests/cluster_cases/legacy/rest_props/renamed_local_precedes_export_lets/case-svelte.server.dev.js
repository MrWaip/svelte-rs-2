App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	const $$restProps = $.rest_props($$sanitized_props, [
		"class",
		"data",
		"scale"
	]);
	$$renderer.component(($$renderer) => {
		let className = $.fallback($$props["class"], "");
		let data = $$props["data"];
		let scale = $.fallback($$props["scale"], 1);
		$$renderer.push(`<div${$.attributes({
			...$$restProps,
			class: $.clsx(className)
		})}>`);
		$.push_element($$renderer, "div", 9, 0);
		$$renderer.push(`${$.escape(data)}${$.escape(scale)}</div>`);
		$.pop_element();
		$.bind_props($$props, {
			class: className,
			data,
			scale
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
