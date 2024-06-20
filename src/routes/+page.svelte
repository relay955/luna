<script lang="ts">
    import type { FileItem } from "../logics/fileitem";
  import LeftItemList from "./LeftItemList.svelte";
    import ListView from "./ListView.svelte";
  import TitleBar from "./TitleBar.svelte";
  import TopMenu from "./TopMenu.svelte";
  import { invoke } from "@tauri-apps/api/tauri"

  let searchbarMode : "filter"|"search"|"path" = "path";
  let directory = "c:/";
  let fileItems:FileItem[] = [];

  $:{
    let options = {
      dir:directory,
      sortBy:"name",
      sortDirection:"asc",
      groupingMode:"folder",
      search:"",
      filter:""
    }
    invoke("get_file_list",options).then((res)=>{
      console.log(res)
      fileItems = res as FileItem[];
    });
  }

</script>

<div class="container">
  <TitleBar />
  <div style="margin-top: 30px;"></div>
  <TopMenu searchbarMode={searchbarMode} />
  <div class="inner-container">
    <LeftItemList />
    <div class="file-list">
      <ListView fileItems={fileItems}/>
    </div>
  </div>
</div>

<style>
  .container{
    height:100%;
    display: flex;
    flex-direction: column;
  }
  .inner-container {
    width: 100%;
    height: 0;
    flex-grow: 1;
    display: flex;
  }
  .file-list{
    flex-grow: 1;
    overflow-y: hidden;
  }
</style>
