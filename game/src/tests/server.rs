use crate::server::*;

#[test]
fn fs_test_1() {
    let mut fs = FileSystem::default();
    fs.files.push(File::new("test.txt".to_string()));
    fs.files[0].set_contents("i am a contents :D".to_string());
    assert_eq!(fs.files[0].get_contents(), &"i am a contents :D".to_string())
}