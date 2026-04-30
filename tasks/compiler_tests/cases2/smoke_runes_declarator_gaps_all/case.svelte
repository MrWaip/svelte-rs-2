<script>
	let { rawProp, rawPropObj } = $props();

	var safeCount = $state(0);
	var safeObj = $state({ x: 0 });

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
</script>

{safeCount}
{safeObj.x}
{rawProp}
{rawPropObj.x}

{safeCount = 1}
{safeCount += 2}
{safeCount++}
{++safeCount}
{safeCount &&= 3}

{safeObj.x = 1}
{safeObj.x += 2}
{safeObj.x++}

{rawProp = 8}
{rawProp += 9}
{rawProp++}
{rawProp &&= 10}

{rawPropObj.x = 11}
{rawPropObj.x += 12}
{rawPropObj.x++}

<button onclick={script_ops}>run</button>
