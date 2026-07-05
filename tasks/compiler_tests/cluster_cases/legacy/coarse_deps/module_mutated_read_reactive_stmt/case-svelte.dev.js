import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
let count = 0;
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[12, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const doubled = $.mutable_source();
	function bump() {
		count = count + 1;
	}
	$.legacy_pre_effect(() => {}, () => {
		$.set(doubled, count);
	});
	$.legacy_pre_effect_reset();
	var $$exports = {
		...$.legacy_api(),
		get bump() {
			return bump;
		}
	};
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(doubled)));
	$.append($$anchor, p);
	$.bind_prop($$props, "bump", bump);
	return $.pop($$exports);
}
