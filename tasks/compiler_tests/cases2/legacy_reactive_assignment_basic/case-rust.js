import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const sum = $.mutable_source();
	let label = $.prop($$props, "label", 8, "sum");
	let a = 1;
	let b = 2;
	$.legacy_pre_effect(() => {}, () => {
		$.set(sum, a + b);
	});
	$.legacy_pre_effect(() => ($.deep_read_state(label()), $.get(sum)), () => {
		console.log(`${label()}: ${$.get(sum)}`);
	});
	$.legacy_pre_effect(() => $.get(sum), () => {
		((param) => {
			via_iife = param * 2;
		})($.get(sum));
	});
	$.legacy_pre_effect_reset();
	$.init();
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$.get(sum) ?? ""}-${via_iife ?? ""}`));
	$.append($$anchor, p);
	$.pop();
}
