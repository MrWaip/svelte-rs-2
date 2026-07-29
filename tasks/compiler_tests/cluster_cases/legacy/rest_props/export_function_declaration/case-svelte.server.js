import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	const $$restProps = $.rest_props($$sanitized_props, ["formatTitle", "Helper"]);
	function formatTitle(prefix) {
		return prefix + "!";
	}
	class Helper {}
	$$renderer.push(`<div${$.attributes({ ...$$restProps })}>${$.escape(formatTitle("a"))}</div>`);
	$.bind_props($$props, {
		formatTitle,
		Helper
	});
}
