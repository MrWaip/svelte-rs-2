import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>increment</button>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $count = () => $.store_get($.get(count), "$count", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let base = $.prop($$props, "base", 8, 0);
	let count = $.mutable_source(0);
	$.legacy_pre_effect(() => $.deep_read_state(base()), () => {
		$.store_unsub($.set(count, base() * 2), "$count", $$stores);
	});
	$.legacy_pre_effect(() => {}, () => {
		$.store_set($.get(count), 1);
	});
	$.legacy_pre_effect_reset();
	var button = root();
	$.delegated("click", button, () => $.update(count));
	$.append($$anchor, button);
	$.pop();
	$$cleanup();
}
$.delegate(["click"]);
