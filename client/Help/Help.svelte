<script>
  import Drawer, { AppContent, Content } from "@smui/drawer";
  import List, { Item, Text, Graphic } from "@smui/list";
  import Menu from "../common/Menu.svelte"

  // Icons for Components (my convention, prefix with 'I' as in IComponent)
  import ICheck from "svelte-material-icons/Check.svelte";
  import ITransitConnectionVariant from "svelte-material-icons/TransitConnectionVariant.svelte";
  import IDiskSpace from "svelte-material-icons/Database.svelte";
  import IJournal from "svelte-material-icons/MathLog.svelte";
  import IUptime from "svelte-material-icons/Clock.svelte";
  //import IPowerCycle from "svelte-material-icons/PowerCycle.svelte";
  import IPowerCycle from "svelte-material-icons/Flash.svelte";
  import ILaunch from "svelte-material-icons/Rocket.svelte";
  import ISettings from "svelte-material-icons/WrenchOutline.svelte";
  import IReload from "svelte-material-icons/Reload.svelte";
  //import Plug from "svelte-material-icons/PowerPlug.svelte";
  //import PlugOff from "svelte-material-icons/PowerPlugOff.svelte";
  import IPerformance from "svelte-material-icons/ChartLine.svelte";
  import IStatus from "svelte-material-icons/ShieldCheckOutline.svelte";
  import IFoo from "svelte-material-icons/ChartLine.svelte";

  // Components
  import DiskSpace from "./DiskSpace.svelte";
  import Status from "./Status.svelte";

  let clicked;
  const menuClick = (e) => { 
    clicked = e.detail; 
    console.log("Help: menuClick invoked: ", clicked);
  }

</script>

<div class="drawer-container">
  <Drawer>
    <Content>
      <List>
        <Menu on:menuClick={menuClick} />
      </List>
    </Content>
  </Drawer>

  <AppContent class="app-content">
    <main class="main-content">
      {#if clicked == "Disk Space"}
        <DiskSpace />
      {:else if clicked == "Status"}
        <Status />
      {:else if clicked == ""}
        Device Manager Control Panel
      {:else}
        <p>you clicked {clicked}</p>
      {/if}
    </main>
  </AppContent>
</div>

<style>
  /* These classes are only needed because the
      drawer is in a container on the page. */
  .drawer-container {
    position: relative;
    display: flex;
    /*height: 350px;*/
    /*max-width: 600px;*/
    border: 1px solid
      var(--mdc-theme-text-hint-on-background, rgba(0, 0, 0, 0.1));
    overflow: hidden;
    z-index: 0;
  }

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
