import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	const $$sanitized_props = $.legacy_rest_props($$props, [
		"children",
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	const $$restProps = $.legacy_rest_props($$sanitized_props, ["api", "a"]);
	$.push($$props, false, App);
	let a = $.prop($$props, "a", 8);
	const api = () => 1;
	const rest = $$restProps;
	var $$exports = {
		...$.legacy_api(),
		get api() {
			return api;
		}
	};
	$.bind_prop($$props, "api", api);
	return $.pop($$exports);
}
