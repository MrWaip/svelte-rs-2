import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[10, 4]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $items = () => ($.validate_store(items, "items"), $.store_get(items, "$items", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	let prop = $.prop($$props, "prop", 8);
	const items = writable([]);
	let show = true;
	var $$exports = { ...$.legacy_api() };
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var p = root();
			var text = $.child(p);
			$.reset(p);
			$.template_effect(() => $.set_text(text, `shown ${prop() ?? ""}`));
			$.append($$anchor, p);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($items(), $.untrack(() => show && $items().length > 0)) $$render(consequent);
		}), "if", App, 9, 0);
	}
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
