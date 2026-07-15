import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	const $$restProps = $.rest_props($$sanitized_props, [
		"class",
		"data",
		"scale"
	]);
	let className = $.fallback($$props["class"], "");
	let data = $$props["data"];
	let scale = $.fallback($$props["scale"], 1);
	$$renderer.push(`<div${$.attributes({
		...$$restProps,
		class: $.clsx(className)
	})}>${$.escape(data)}${$.escape(scale)}</div>`);
	$.bind_props($$props, {
		class: className,
		data,
		scale
	});
}
