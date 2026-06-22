import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	const $$sanitized_props = $.legacy_rest_props($$props, [
		"children",
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	const $$restProps = $.legacy_rest_props($$sanitized_props, ["x"]);
	$.push($$props, false);
	let x = $.prop($$props, "x", 8);
	$.legacy_pre_effect(() => ($.deep_read_state($$restProps), $.deep_read_state(x())), () => {
		$$restProps, x();
	});
	$.legacy_pre_effect_reset();
	$.pop();
}
