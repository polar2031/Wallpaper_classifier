#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        path::{Path, PathBuf},
    };
    use uuid;

    use crate::{check_dimension, get_file_paths, get_image_dimension, take_action};

    #[test]
    fn test_get_file_paths() {
        let path = "src/test_folder";
        let paths = get_file_paths(path);
        assert_eq!(paths.len(), 4);
    }

    #[test]
    fn test_get_image_dimension() {
        let path = Path::new("src/test_folder/500x500.png");
        let dimension = get_image_dimension(path);
        assert_eq!(dimension, Some((500, 500)));
    }

    #[test]
    fn test_get_image_dimension_not_image() {
        let path = Path::new("src/test_folder/not_image.txt");
        let dimension = get_image_dimension(path);
        assert_eq!(dimension, None);
    }

    #[test]
    fn test_get_image_dimension_not_exist() {
        let path = Path::new("src/test_folder/not_exist.png");
        let dimension = get_image_dimension(path);
        assert_eq!(dimension, None);
    }

    #[test]
    fn test_check_dimension() {
        let min_width = Some(100);
        let min_height = Some(100);
        let max_width = Some(1000);
        let max_height = Some(1000);
        let width = 500;
        let height = 500;
        let result = check_dimension(min_width, min_height, max_width, max_height, width, height);
        assert_eq!(result, true);
    }

    #[test]
    fn test_check_dimension_without_min() {
        let min_width = None;
        let min_height = None;
        let max_width = Some(1000);
        let max_height = Some(1000);
        let width = 500;
        let height = 500;
        let result = check_dimension(min_width, min_height, max_width, max_height, width, height);
        let width = 1500;
        let height = 1500;
        assert_eq!(result, true);
        let result = check_dimension(min_width, min_height, max_width, max_height, width, height);
        assert_eq!(result, false);
    }

    #[test]
    fn test_check_dimension_without_max() {
        let min_width = Some(100);
        let min_height = Some(100);
        let max_width = None;
        let max_height = None;
        let width = 500;
        let height = 500;
        let result = check_dimension(min_width, min_height, max_width, max_height, width, height);
        assert_eq!(result, true);
        let width = 50;
        let height = 50;
        let result = check_dimension(min_width, min_height, max_width, max_height, width, height);
        assert_eq!(result, false);
    }

    fn create_dummy_file(file_path_string: &str) -> Result<(), std::io::Error> {
        // create dummy file in source folder
        // if parent folder not exists, create it
        let file_path = Path::new(file_path_string);
        let parent_folder = file_path.parent().unwrap();
        if !parent_folder.exists() {
            std::fs::create_dir_all(parent_folder)?;
        }
        let _ = File::create(file_path)?;
        Ok(())
    }

    #[test]
    fn test_take_action_copy() {
        let action = "copy";

        // create dummy file in source folder
        let uuid_string1: String = uuid::Uuid::new_v4().to_string();
        let source_folder_path_string = &format!("src/test_folder/{}", uuid_string1);
        let source_folder_path = PathBuf::from(source_folder_path_string);
        let source_file_path_string = &format!("src/test_folder/{}/sample.txt", uuid_string1);
        let source_file_path = PathBuf::from(source_file_path_string);
        let _ = create_dummy_file(source_file_path_string);

        let uuid_string2: String = uuid::Uuid::new_v4().to_string();
        let dest_folder_path_string = &format!("src/test_folder/{}", uuid_string2);
        let dest_folder_path = PathBuf::from(dest_folder_path_string);
        let dest_file_path_string = &format!("src/test_folder/{}/sample.txt", uuid_string2);
        let dest_file_path = PathBuf::from(dest_file_path_string);
        take_action(action, &source_file_path, Some(&dest_folder_path));
        assert_eq!(dest_file_path.exists(), true);

        // clear test files
        let _ = std::fs::remove_dir_all(source_folder_path);
        let _ = std::fs::remove_dir_all(dest_folder_path);
    }

    #[test]
    fn test_take_action_move() {
        let action = "move";

        // create dummy file in source folder
        let uuid_string1: String = uuid::Uuid::new_v4().to_string();
        let source_folder_path_string = &format!("src/test_folder/{}", uuid_string1);
        let source_folder_path = PathBuf::from(source_folder_path_string);
        let source_file_path_string = &format!("src/test_folder/{}/sample.txt", uuid_string1);
        let source_file_path = PathBuf::from(source_file_path_string);
        let _ = create_dummy_file(source_file_path_string);

        let uuid_string2: String = uuid::Uuid::new_v4().to_string();
        let dest_folder_path_string = &format!("src/test_folder/{}", uuid_string2);
        let dest_folder_path = PathBuf::from(dest_folder_path_string);
        let dest_file_path_string = &format!("src/test_folder/{}/sample.txt", uuid_string2);
        let dest_file_path = PathBuf::from(dest_file_path_string);
        take_action(action, &source_file_path, Some(&dest_folder_path));
        assert_eq!(dest_file_path.exists(), true);
        assert_eq!(source_file_path.exists(), false);

        // clear test files
        let _ = std::fs::remove_dir_all(source_folder_path);
        let _ = std::fs::remove_dir_all(dest_folder_path);
    }

    #[test]
    fn test_take_action_delete() {
        // create dummy file in source folder
        let uuid_string: String = uuid::Uuid::new_v4().to_string();
        let source_folder_path_string = &format!("src/test_folder/{}", uuid_string);
        let source_folder_path = PathBuf::from(source_folder_path_string);
        let source_file_path_string = &format!("src/test_folder/{}/sample.txt", uuid_string);
        let source_file_path = PathBuf::from(source_file_path_string);
        let _ = create_dummy_file(source_file_path_string);

        take_action("delete", &source_file_path, None);
        assert_eq!(source_file_path.exists(), false);

        // clear test files
        let _ = std::fs::remove_dir_all(source_folder_path);
    }
}
