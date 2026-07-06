App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let obj = { field: { x: 0 } };
		let deep = { a: { b: { c: { x: 0 } } } };
		function script_ops() {
			obj.field.x = 1;
			obj.field.x += 2;
			obj.field.x++;
			++obj.field.x;
			obj.field.x &&= 3;
			deep.a.b.c.x = 4;
			deep.a.b.c.x += 5;
			deep.a.b.c.x++;
		}
		$$renderer.push(`<!---->${$.escape(obj.field.x)}
${$.escape(deep.a.b.c.x)}

${$.escape(obj.field.x = 1)}
${$.escape(obj.field.x += 2)}
${$.escape(obj.field.x++)}
${$.escape(++obj.field.x)}

${$.escape(deep.a.b.c.x = 4)}
${$.escape(deep.a.b.c.x++)} <button>`);
		$.push_element($$renderer, "button", 29, 0);
		$$renderer.push(`run</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
