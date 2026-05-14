import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { onMount } from "svelte";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const derived_count = $.mutable_source();
	let flag = $.mutable_source(false);
	onMount(() => {
		const cb = () => {
			$.set(flag, true);
		};
		cb();
	});
	$.legacy_pre_effect(() => $.get(flag), () => {
		$.set(derived_count, (() => {
			if ($.get(flag)) {
				return 1;
			}
			return 0;
		})());
	});
	$.legacy_pre_effect_reset();
	$.init();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(derived_count)));
	$.append($$anchor, p);
	$.pop();
}
