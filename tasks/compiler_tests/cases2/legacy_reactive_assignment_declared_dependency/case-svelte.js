import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const doubled = $.mutable_source();
	const total = $.mutable_source();
	let count = $.mutable_source(1);
	var step = $.mutable_source(2);
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
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(doubled) ?? ""}-${$.get(total) ?? ""}`));
	$.delegated("click", button, bump);
	$.append($$anchor, button);
	$.pop();
}
$.delegate(["click"]);
