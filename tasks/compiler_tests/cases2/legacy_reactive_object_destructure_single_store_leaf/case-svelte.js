import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $value = () => $.store_get($.get(value), "$value", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const value = $.mutable_source();
	let store = $.prop($$props, "store", 8);
	$.legacy_pre_effect(() => ($.get(value), $.deep_read_state(store())), () => {
		(($$value) => {
			$.store_unsub($.set(value, $$value.value), "$value", $$stores);
		})(store());
	});
	$.legacy_pre_effect_reset();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $value()));
	$.append($$anchor, p);
	$.pop();
	$$cleanup();
}
