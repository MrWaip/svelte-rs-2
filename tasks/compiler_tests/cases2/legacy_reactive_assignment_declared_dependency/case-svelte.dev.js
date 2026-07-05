import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[15, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const doubled = $.mutable_source();
	const total = $.mutable_source();
	let count = $.tag($.mutable_source(1), "count");
	var step = $.tag($.mutable_source(2), "step");
	function bump() {
		$.set(count, $.get(count) + 1);
		$.set(step, $.safe_get(step) + 1);
	}
	$.legacy_pre_effect(() => $.get(count), () => {
		$.set(doubled, $.get(count) * 2);
	});
	$.legacy_pre_effect(() => ($.get(doubled), $.safe_get(step)), () => {
		$.set(total, $.get(doubled) + $.safe_get(step));
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(doubled) ?? ""}-${$.get(total) ?? ""}`));
	$.delegated("click", button, bump);
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
