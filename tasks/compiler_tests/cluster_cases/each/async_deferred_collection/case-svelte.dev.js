import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[9, 1]]);
var root_1 = $.add_locations($.from_html(`<p>empty</p>`), App[$.FILENAME], [[11, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function delay(value) {
		return Promise.resolve(value);
	}
	var loaded;
	var $$promises = $.run([async () => loaded = (await $.track_reactivity_loss(delay([1, 2])))()]);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.async(node, [$$promises[0]], void 0, (node) => {
		$.add_svelte_meta(() => $.each(node, 17, () => loaded, $.index, ($$anchor, item) => {
			var p = root();
			var text = $.child(p, true);
			$.reset(p);
			$.template_effect(() => $.set_text(text, $.get(item)));
			$.append($$anchor, p);
		}, ($$anchor) => {
			var p_1 = root_1();
			$.append($$anchor, p_1);
		}), "each", App, 8, 0);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
