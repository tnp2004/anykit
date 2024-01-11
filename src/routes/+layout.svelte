<script>
  import "../app.css";

  import { Toast, initializeStores } from "@skeletonlabs/skeleton";

  initializeStores();
</script>

<Toast />
<div class="px-5 py-2">
  <slot />
</div>
