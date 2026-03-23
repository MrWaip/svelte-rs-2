<script lang="ts">
import { foo } from './foo';
// this comment should move
import type { Bar } from './bar';
let count = $state(0);
</script>
<p>{count}</p>
