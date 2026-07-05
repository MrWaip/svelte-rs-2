import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
import Inner from "./Inner.svelte";
var root = $.add_locations($.from_html(`<div>hi</div>`), App[$.FILENAME], [[14, 12]]);
var root_1 = $.add_locations($.from_html(`<div slot="icon"><!></div>`), App[$.FILENAME], [[12, 4]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $meta = () => ($.validate_store(meta(), "meta"), $.store_get(meta(), "$meta", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	let meta = $.prop($$props, "meta", 24, () => writable({ disabled: false }));
	let x;
	let component = Inner;
	var $$exports = { ...$.legacy_api() };
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.component(node, () => component, ($$anchor, $$component) => {
		$$component($$anchor, { $$slots: { icon: ($$anchor, $$slotProps) => {
			var div = root_1();
			var node_1 = $.child(div);
			{
				var consequent = ($$anchor) => {
					var div_1 = root();
					$.append($$anchor, div_1);
				};
				$.add_svelte_meta(() => $.if(node_1, ($$render) => {
					if ($meta(), $.untrack(() => x && !$meta().disabled)) $$render(consequent);
				}), "if", App, 13, 8);
			}
			$.reset(div);
			$.append($$anchor, div);
		} } });
	}), "component", App, 11, 0, { componentTag: "svelte:component" });
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
