import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy"
]);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const body = ($$anchor) => {
		var fragment = $.comment();
		var node = $.first_child(fragment);
		{
			var consequent = ($$anchor) => {
				var fragment_1 = $.comment();
				var node_1 = $.first_child(fragment_1);
				$.component(node_1, () => $$props.Inner, ($$anchor, props_Inner) => {
					props_Inner($$anchor, {});
				});
				$.append($$anchor, fragment_1);
			};
			$.if(node, ($$render) => {
				if ($$props.Inner) $$render(consequent);
			});
		}
		$.append($$anchor, fragment);
	};
	let props = $.rest_props($$props, rest_excludes);
	{
		let $0 = $.derived(() => $$props.show ? body : undefined);
		Child($$anchor, { get icon() {
			return $.get($0);
		} });
	}
	$.pop();
}
