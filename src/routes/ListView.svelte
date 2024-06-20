<script lang="ts">
    import type { FileItem } from "../logics/fileitem";
    import { beutifyFileSize } from "../utils/filesizeBeutifier";

    export let fileItems = [] as FileItem[];

</script>

<div class="root">
    <table>
        <thead>
        <tr>
            <th>Name</th>
            <th>Edit Date</th>
            <th>Size</th>
            <th>Type</th>
        </tr>
        </thead>
        <tbody>
            {#each fileItems as fileItem}
            <tr>
            <td class="td-name">{fileItem.name}</td>
            <td class="td-date">{fileItem.edit_date}</td>
            <td class="td-size">{beutifyFileSize(fileItem.size)}</td>
            <td class="td-type">{fileItem.file_type}</td>
            </tr>
            {/each}
        </tbody>
    </table>
</div>

<style lang="scss">
    .root{
        height: 100%;
        overflow-y: auto;
    }
    table {
        width: 100%;
        border-collapse: collapse;
    }

    th, td {
        padding: 2px 4px;
        text-align: left;
    }

    tr > th {
        border-bottom: 1px solid #ddd;
        position: sticky;
        top: 0;
    }

    th {
        background-color: #f5f5f5;
        font-size: 14px;
    }

    tr:nth-child(even) {
        background-color: #f5f5f5;
    }


    .td-name {
        width: 400px;
        font-size: 12px;
    }
    .td-date {
        width: 200px;
        font-size: 12px;
    }
    .td-size{
        width: 100px;
        font-size: 12px;
    }


</style>