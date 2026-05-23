import * as $ from "svelte/internal/client";
import Outer from "./Outer.svelte";
import Img from "./Img.svelte";
import Btn from "./Btn.svelte";
var root_4 = $.from_html(`<span>a</span>`);
export default function App($$anchor) {
	let cond = true;
	Outer($$anchor, { $$slots: {
		image: ($$anchor, $$slotProps) => {
			Img($$anchor, { slot: "image" });
		},
		action: ($$anchor, $$slotProps) => {
			Btn($$anchor, {
				slot: "action",
				children: ($$anchor, $$slotProps) => {
					var fragment_3 = $.comment();
					var node = $.first_child(fragment_3);
					{
						var consequent = ($$anchor) => {
							var span = root_4();
							$.append($$anchor, span);
						};
						var alternate = ($$anchor) => {
							var text = $.text("b");
							$.append($$anchor, text);
						};
						$.if(node, ($$render) => {
							if (cond) $$render(consequent);
							else $$render(alternate, -1);
						});
					}
					$.append($$anchor, fragment_3);
				},
				$$slots: { default: true }
			});
		}
	} });
}
