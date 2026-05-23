import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $error = () => $.store_get($.get(error), "$error", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const error = $.mutable_source();
	let id = $.prop($$props, "id", 8);
	$.legacy_pre_effect(() => ($.get(error), $.deep_read_state(id())), () => {
		(($$value) => {
			$.store_unsub($.set(error, $$value.error), "$error", $$stores);
		})(make(id()));
	});
	$.legacy_pre_effect_reset();
	Child($$anchor, {
		get value() {
			$.mark_store_binding();
			return $error();
		},
		set value($$value) {
			$.store_set($.get(error), $$value);
		},
		$$legacy: true
	});
	$.pop();
	$$cleanup();
}
