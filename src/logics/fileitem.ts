export interface FileItem {
    name: string;
    size: number;
    file_type: string;
    edit_date: string;
    hidden:boolean;
    full_path:string;
    decrypted_name?:string;
}
