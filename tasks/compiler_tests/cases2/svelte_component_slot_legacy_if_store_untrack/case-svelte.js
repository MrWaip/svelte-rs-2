import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
import Inner from "./Inner.svelte";
var root = $.from_html(`<div>hi</div>`);
var root_1 = $.from_html(`<div slot="icon"><!></div>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $meta = () => $.store_get(meta(), "$meta", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let meta = $.prop($$props, "meta", 24, () => writable({ disabled: false }));
	let x;
	let component = Inner;
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.component(node, () => component, ($$anchor, $$component) => {
		$$component($$anchor, { $$slots: { icon: ($$anchor, $$slotProps) => {
			var div = root_1();
			var node_1 = $.child(div);
			{
				var consequent = ($$anchor) => {
					var div_1 = root();
					$.append($$anchor, div_1);
				};
				$.if(node_1, ($$render) => {
					if ($meta(), $.untrack(() => x && !$meta().disabled)) $$render(consequent);
				});
			}
			$.reset(div);
			$.append($$anchor, div);
		} } });
	});
	$.append($$anchor, fragment);
	$.pop();
	$$cleanup();
}
