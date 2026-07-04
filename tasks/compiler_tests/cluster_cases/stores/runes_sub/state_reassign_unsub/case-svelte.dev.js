App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.add_locations($.from_html(` <button>remove</button>`, 1), App[$.FILENAME], [[8, 1]]);
var root_1 = $.add_locations($.from_html(`<button>add</button>`), App[$.FILENAME], [[10, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $watcherA = () => ($.validate_store($.get(watcherA), "watcherA"), $.store_get($.get(watcherA), "$watcherA", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	let watcherA = $.tag($.state(void 0), "watcherA");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var fragment_1 = root();
			var text = $.first_child(fragment_1);
			var button = $.sibling(text);
			$.template_effect(() => $.set_text(text, `${$watcherA() ?? ""} `));
			$.delegated("click", button, function click() {
				return $.store_unsub($.set(watcherA, null), "$watcherA", $$stores);
			});
			$.append($$anchor, fragment_1);
		};
		var alternate = ($$anchor) => {
			var button_1 = root_1();
			$.delegated("click", button_1, function click_1() {
				return $.store_unsub($.set(watcherA, writable(0), true), "$watcherA", $$stores);
			});
			$.append($$anchor, button_1);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($.get(watcherA)) $$render(consequent);
			else $$render(alternate, -1);
		}), "if", App, 6, 0);
	}
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
$.delegate(["click"]);
