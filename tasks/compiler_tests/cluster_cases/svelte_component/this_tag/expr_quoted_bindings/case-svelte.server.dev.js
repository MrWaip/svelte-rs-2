App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Foo from "./Foo.svelte";
import Bar from "./Bar.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let x = $$props["x"];
		let y = $$props["y"];
		let z = $$props["z"];
		let $$settled = true;
		let $$inner_renderer;
		function $$render_inner($$renderer) {
			if (x ? Foo : Bar) {
				$$renderer.push("<!--[-->");
				(x ? Foo : Bar)($$renderer, {
					get y() {
						return y;
					},
					set y($$value) {
						y = $$value;
						$$settled = false;
					},
					get z() {
						return z;
					},
					set z($$value) {
						z = $$value;
						$$settled = false;
					}
				});
				$$renderer.push("<!--]-->");
			} else {
				$$renderer.push("<!--[!-->");
				$$renderer.push("<!--]-->");
			}
		}
		do {
			$$settled = true;
			$$inner_renderer = $$renderer.copy();
			$$render_inner($$inner_renderer);
		} while (!$$settled);
		$$renderer.subsume($$inner_renderer);
		$.bind_props($$props, {
			x,
			y,
			z
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
