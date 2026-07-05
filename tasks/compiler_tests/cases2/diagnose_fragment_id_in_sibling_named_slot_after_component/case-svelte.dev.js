App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Outer from "./Outer.svelte";
import Img from "./Img.svelte";
import Btn from "./Btn.svelte";
var root = $.add_locations($.from_html(`<span>a</span>`), App[$.FILENAME], [[11, 18]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let cond = true;
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Outer($$anchor, { $$slots: {
		image: ($$anchor, $$slotProps) => {
			$.add_svelte_meta(() => Img($$anchor, { slot: "image" }), "component", App, 9, 4, { componentTag: "Img" });
		},
		action: ($$anchor, $$slotProps) => {
			$.add_svelte_meta(() => Btn($$anchor, {
				slot: "action",
				children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
					var fragment_3 = $.comment();
					var node = $.first_child(fragment_3);
					{
						var consequent = ($$anchor) => {
							var span = root();
							$.append($$anchor, span);
						};
						var alternate = ($$anchor) => {
							var text = $.text("b");
							$.append($$anchor, text);
						};
						$.add_svelte_meta(() => $.if(node, ($$render) => {
							if (cond) $$render(consequent);
							else $$render(alternate, -1);
						}), "if", App, 11, 8);
					}
					$.append($$anchor, fragment_3);
				}),
				$$slots: { default: true }
			}), "component", App, 10, 4, { componentTag: "Btn" });
		}
	} }), "component", App, 8, 0, { componentTag: "Outer" });
	return $.pop($$exports);
}
