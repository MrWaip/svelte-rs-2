import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { onMount } from "svelte";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[15, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const derived_count = $.mutable_source();
	let flag = $.tag($.mutable_source(false), "flag");
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
	var $$exports = { ...$.legacy_api() };
	$.init();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(derived_count)));
	$.append($$anchor, p);
	return $.pop($$exports);
}
