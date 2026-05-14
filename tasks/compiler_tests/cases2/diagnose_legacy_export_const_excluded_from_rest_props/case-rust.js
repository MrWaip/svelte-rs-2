import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	const $$sanitized_props = $.legacy_rest_props($$props, [
		"children",
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	const $$restProps = $.legacy_rest_props($$sanitized_props, ["api", "a"]);
	$.push($$props, false);
	let a = $.prop($$props, "a", 8);
	const api = () => 1;
	const rest = $$restProps;
	var $$exports = { api };
	$.bind_prop($$props, "api", api);
	return $.pop($$exports);
}
