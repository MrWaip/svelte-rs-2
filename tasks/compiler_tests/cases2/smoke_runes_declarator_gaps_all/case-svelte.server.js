import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { rawProp, rawPropObj } = $$props;
		var safeCount = 0;
		var safeObj = { x: 0 };
		function script_ops() {
			safeCount = 1;
			safeCount += 2;
			safeCount++;
			++safeCount;
			safeCount &&= 3;
			safeObj.x = 1;
			safeObj.x += 2;
			safeObj.x++;
			rawProp = 8;
			rawProp += 9;
			rawProp++;
			++rawProp;
			rawProp &&= 10;
			rawPropObj.x = 11;
			rawPropObj.x += 12;
			rawPropObj.x++;
		}
		$$renderer.push(`<!---->${$.escape(safeCount)}
${$.escape(safeObj.x)}
${$.escape(rawProp)}
${$.escape(rawPropObj.x)}

${$.escape(safeCount = 1)}
${$.escape(safeCount += 2)}
${$.escape(safeCount++)}
${$.escape(++safeCount)}
${$.escape(safeCount &&= 3)}

${$.escape(safeObj.x = 1)}
${$.escape(safeObj.x += 2)}
${$.escape(safeObj.x++)}

${$.escape(rawProp = 8)}
${$.escape(rawProp += 9)}
${$.escape(rawProp++)}
${$.escape(rawProp &&= 10)}

${$.escape(rawPropObj.x = 11)}
${$.escape(rawPropObj.x += 12)}
${$.escape(rawPropObj.x++)} <button>run</button>`);
	});
}
