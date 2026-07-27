import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>add</button> <!>`, 1), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let items = $.tag_proxy($.proxy([1, 2]), "items");
	function delay(value) {
		return Promise.resolve(value);
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.add_svelte_meta(() => $.each(node, 17, () => items, $.index, ($$anchor, item) => {
		$.next();
		var text = $.text();
		$.template_effect(($0) => $.set_text(text, $0), void 0, [async () => (await $.track_reactivity_loss(delay($.get(item))))()]);
		$.append($$anchor, text);
	}), "each", App, 11, 0);
	$.delegated("click", button, function click() {
		return items.push(items.length);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
