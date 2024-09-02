<script lang="ts">
  import Input from "./common/Input.svelte";
  import FaSearch from "svelte-icons/fa/FaSearch.svelte";
  import IconButton from "./common/IconButton.svelte";
  import FaFilter from "svelte-icons/fa/FaFilter.svelte";
  import FaFolderOpen from "svelte-icons/fa/FaFolderOpen.svelte";
  import ButtonGroup from "./common/ButtonGroup.svelte";
  import FaListUl from "svelte-icons/fa/FaListUl.svelte";
  import FaThLarge from "svelte-icons/fa/FaThLarge.svelte";
  import FaTicketAlt from "svelte-icons/fa/FaTicketAlt.svelte";
  import FaFile from 'svelte-icons/fa/FaFile.svelte'
  import FaArrowLeft from 'svelte-icons/fa/FaArrowLeft.svelte'
  import FaLock from 'svelte-icons/fa/FaLock.svelte'
  import FaUnlock from 'svelte-icons/fa/FaUnlock.svelte'
  import SecureInput from "./common/SecureInput.svelte";

  export let searchbarMode: "filter" | "search" | "path" = "filter";
  export let protectionMode: "protected" | "normal" = "normal";
  export let directory: string;
  export let onClickHistoryBack: () => void;
  export let onClickProtectionMode: (protectKey:string) => void;

  let temporaryDirectory: string;
  $:temporaryDirectory = directory;

  let protectKey = "";

  const onEnterSearchInput = (e: KeyboardEvent) => {
    if (e.key != "Enter") return;
    directory = temporaryDirectory;
  }

  const onClickProtectionModeProxy = () => {
    onClickProtectionMode(protectKey);
    protectKey = "";
  }

</script>

<div class="topmenu">
  <div class="container">
    <IconButton style="margin-right:10px;" onClick={onClickHistoryBack}>
      <FaArrowLeft/>
    </IconButton>
    <IconButton
        selected={searchbarMode === "filter"}
        style="margin-right:5px"
    >
      <FaFilter/>
    </IconButton>
    <IconButton
        selected={searchbarMode === "search"}
        style="margin-right:5px"
    >
      <FaSearch/>
    </IconButton>
    <IconButton
        selected={searchbarMode === "path"}
        style="margin-right:5px"
    >
      <FaFolderOpen/>
    </IconButton>
    <Input style="flex:1" bind:value={temporaryDirectory} keydown={onEnterSearchInput}/>
  </div>
  <div class="container" style="margin-top:5px">
    <ButtonGroup name="view mode">
      <IconButton large style="margin-right:5px;">
        <FaListUl/>
      </IconButton>
      <IconButton large style="margin-right:5px;">
        <FaThLarge/>
      </IconButton>
      <IconButton large style="margin-right:5px;">
        <FaTicketAlt/>
      </IconButton>
    </ButtonGroup>
    <ButtonGroup name="grouping" style="margin-left:5px">
      <IconButton large style="margin-right:5px;">
        ALL
      </IconButton>
      <IconButton large style="margin-right:5px;">
        <FaFolderOpen/>
      </IconButton>
      <IconButton large style="margin-right:5px;">
        <FaFile/>
      </IconButton>
    </ButtonGroup>
    <ButtonGroup name="protection mode" style="margin-left:5px">
      {#if protectionMode === "normal"}
        <SecureInput style="margin-left:5px; margin-right:5px; width: 100px; height: 15px"
               bind:value={protectKey}/>
        <IconButton large style="margin-right:5px;" onClick={onClickProtectionModeProxy}>
          <FaLock/>
        </IconButton>
      {/if}
      {#if protectionMode === "protected"}
        <div class="protect-status"> ACTIVE </div>
        <IconButton large style="margin-right:5px;" onClick={onClickProtectionModeProxy}>
          <FaUnlock/>
        </IconButton>
      {/if}
    </ButtonGroup>
  </div>
</div>

<style lang="scss">
  .topmenu {
    padding: 4px;
    box-sizing: border-box;
    width: 100%;
    height: 100px;
    box-shadow: #d3d3d3 0px 0px 10px;
    z-index: 1;
    position: relative;
  }

  .container {
    display: flex;
    align-items: center;
  }

  .protect-status {
    background-color: #c73838;
    color: white;
    padding: 2px 5px;
    border-radius: 5px;
    font-size: 10px;
  }
</style>
