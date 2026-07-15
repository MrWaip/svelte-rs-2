import * as $ from "svelte/internal/client";
import Svg from "./Svg.svelte";
var root = $.from_svg(`<path></path>`);
var root_1 = $.from_svg(`<polygon></polygon>`);
var root_2 = $.from_svg(`<!><!>`, 1);
export default function App($$anchor, $$props) {
	let paths = [];
	let polygons = [];
	Svg($$anchor, {
		children: ($$anchor, $$slotProps) => {
			var fragment_1 = $.comment();
			var node = $.first_child(fragment_1);
			$.slot(node, $$props, "default", {}, ($$anchor) => {
				var fragment_2 = root_2();
				var node_1 = $.first_child(fragment_2);
				$.each(node_1, 17, () => paths, $.index, ($$anchor, path) => {
					var path_1 = root();
					$.attribute_effect(path_1, () => ({ ...$.get(path) }));
					$.append($$anchor, path_1);
				});
				var node_2 = $.sibling(node_1);
				$.each(node_2, 17, () => polygons, $.index, ($$anchor, polygon) => {
					var polygon_1 = root_1();
					$.attribute_effect(polygon_1, () => ({ ...$.get(polygon) }));
					$.append($$anchor, polygon_1);
				});
				$.append($$anchor, fragment_2);
			});
			$.append($$anchor, fragment_1);
		},
		$$slots: { default: true }
	});
}
