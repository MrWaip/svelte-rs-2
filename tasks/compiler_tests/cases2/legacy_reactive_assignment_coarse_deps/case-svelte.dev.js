import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p class="x"> </p>`), App[$.FILENAME], [[10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	const $$sanitized_props = $.legacy_rest_props($$props, [
		"children",
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	const $$restProps = $.legacy_rest_props($$sanitized_props, ["items", "extra"]);
	$.push($$props, false, App);
	const prop_total = $.mutable_source();
	const props_items = $.mutable_source();
	const rest_class = $.mutable_source();
	let items = $.prop($$props, "items", 24, () => [{ value: 1 }]);
	let extra = $.prop($$props, "extra", 8, 2);
	$.legacy_pre_effect(() => ($.deep_read_state(items()), $.deep_read_state(extra())), () => {
		$.set(prop_total, items()[0].value + extra());
	});
	$.legacy_pre_effect(() => $.deep_read_state($$sanitized_props), () => {
		$.set(props_items, $$sanitized_props.items[0].value);
	});
	$.legacy_pre_effect(() => $.deep_read_state($$restProps), () => {
		$.set(rest_class, $$restProps.class ?? "none");
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	$.init();
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$.get(prop_total) ?? ""}-${$.get(props_items) ?? ""}-${$.get(rest_class) ?? ""}`));
	$.append($$anchor, p);
	return $.pop($$exports);
}
