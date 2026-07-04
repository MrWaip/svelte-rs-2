import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	const $$sanitized_props = $.legacy_rest_props($$props, [
		"children",
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	const $$restProps = $.legacy_rest_props($$sanitized_props, ["bar"]);
	$.push($$props, false, App);
	const foo = 1;
	var $$exports = {
		...$.legacy_api(),
		get bar() {
			return foo;
		}
	};
	var div = root();
	$.attribute_effect(div, () => ({ ...$$restProps }));
	div.textContent = "1";
	$.append($$anchor, div);
	$.bind_prop($$props, "bar", foo);
	return $.pop($$exports);
}
