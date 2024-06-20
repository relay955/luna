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
  let directoryHistory:string[] = [];

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

  const onDoubleClickFileItem = (fileItem:FileItem) => {
    if(fileItem.file_type == "dir"){
      directoryHistory.push(directory);
      directory = fileItem.full_path;
    }else{
      invoke("open_file",{filePath:fileItem.full_path});
    }
  }

  const onClickHistoryBack = ()=>{
    if(directoryHistory.length == 0) return;
    directory = directoryHistory.pop()!;
  }

</script>

<div class="container">
  <TitleBar />
  <div style="margin-top: 30px;"></div>
  <TopMenu bind:searchbarMode={searchbarMode} bind:directory={directory}
           onClickHistoryBack={onClickHistoryBack}
   />
  <div class="inner-container">
    <LeftItemList />
    <div class="file-list">
      <ListView fileItems={fileItems} onDoubleClickFileItem={onDoubleClickFileItem}/>
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
