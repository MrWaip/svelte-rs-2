import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $isLoading = () => $.store_get($.get(isLoading), "$isLoading", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const run = $.mutable_source();
	const isLoading = $.mutable_source();
	const a = $.mutable_source();
	const b = $.mutable_source();
	const c = $.mutable_source();
	const d = $.mutable_source();
	let onSubmit = $.prop($$props, "onSubmit", 8);
	let pair = $.prop($$props, "pair", 8);
	function withoutConcurrent(fn) {
		return [fn, { subscribe: () => () => {} }];
	}
	function go() {
		$.get(run)();
	}
	$.legacy_pre_effect(() => ($.get(run), $.get(isLoading), $.deep_read_state(onSubmit())), () => {
		(($$value) => {
			var $$array = $.to_array($$value, 2);
			$.set(run, $$array[0]);
			$.store_unsub($.set(isLoading, $$array[1]), "$isLoading", $$stores);
		})(withoutConcurrent(onSubmit()));
	});
	$.legacy_pre_effect(() => ($.get(a), $.get(b), $.get(c), $.get(d), $.deep_read_state(pair())), () => {
		(($$value) => {
			var $$array_1 = $.to_array($$value, 2);
			var $$array_2 = $.to_array($$array_1[0], 2);
			var $$array_3 = $.to_array($$array_1[1], 2);
			$.set(a, $$array_2[0]);
			$.set(b, $$array_2[1]);
			$.set(c, $$array_3[0]);
			$.set(d, $$array_3[1]);
		})(pair());
	});
	$.legacy_pre_effect_reset();
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$isLoading() ? "wait" : "go"}-${$.get(a) ?? ""}-${$.get(b) ?? ""}-${$.get(c) ?? ""}-${$.get(d) ?? ""}`));
	$.event("click", button, go);
	$.append($$anchor, button);
	$.pop();
	$$cleanup();
}
