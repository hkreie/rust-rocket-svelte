<script>
  import Drawer, { AppContent, Content } from "@smui/drawer";
  import List, { Item, Text, Graphic } from "@smui/list";

  // Components
  import DeviceTable from "./DeviceTable.svelte";

  let clicked = "";


</script>


<AppContent class="app-content">
  <main class="main-content">
    <DeviceTable />
  </main>
</AppContent>


<style>

  * :global(.app-content) {
    flex: auto;
    overflow: auto;
    position: relative;
    flex-grow: 1;
  }

  .main-content {
    overflow: auto;
    padding: 16px;
    height: 100%;
    box-sizing: border-box;
  }
</style>
