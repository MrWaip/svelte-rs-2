import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
import { thing } from "./lib";
var root = $.add_locations($.from_html(`<svelte-css-wrapper style="display: contents"><!></svelte-css-wrapper>`, 1), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const m = $.mutable_source();
	let p = $.prop($$props, "p", 8);
	$.legacy_pre_effect(() => $.deep_read_state(p()), () => {
		$.set(m, p());
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var node = $.first_child(fragment);
	{
		let $0 = $.derived_safe_equal(() => ({
			a: $.get(m),
			b: thing
		}));
		$.css_props(node, () => ({ "--color": "red" }));
		Child(node.lastChild, { get config() {
			return $.get($0);
		} });
		$.reset(node);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
