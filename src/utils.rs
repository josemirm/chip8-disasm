// Removes the path part from the file name. Examples:
//      /tmp/foo.io        ->  foo.io
//      C:\folder\bar.io   ->  bar.io
pub fn trim_filename_path(exec_path: &String) -> String {
    let mut ret: String;

    ret = exec_path.rsplit('/').collect::<Vec<&str>>()[0].to_string();
    ret = ret.rsplit('\\').collect::<Vec<&str>>()[0].to_string();

    return ret;
}
