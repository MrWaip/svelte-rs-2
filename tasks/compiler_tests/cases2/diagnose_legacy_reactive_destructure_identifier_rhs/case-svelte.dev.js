import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const a = $.mutable_source();
	const b = $.mutable_source();
	function makePair() {
		return [1, 2];
	}
	const pair = makePair();
	$.legacy_pre_effect(() => ($.get(a), $.get(b)), () => {
		((pair) => {
			var $$array = $.to_array(pair, 2);
			$.set(a, $$array[0]);
			$.set(b, $$array[1]);
		})(pair);
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}`));
	$.append($$anchor, text);
	return $.pop($$exports);
}
