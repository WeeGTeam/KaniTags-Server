use crate::similarity::coloring::color_and_merge_groups;
use kani_domain_api_incoming::similarity_service::{CalculateSimilarityGroupsError, GetSimilarImagesError, SimilarityService};
use kani_domain_api_model::image_id::ImageId;
use kani_domain_api_model::similarity::{SimilarImage, SimilarImagePair};
use kani_domain_api_outgoing::database::similarity_database::SimilarityDatabase;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub mod coloring;

pub struct SimilarityServiceImpl {
    database: Arc<dyn SimilarityDatabase + Sync + Send>,
}

impl SimilarityService for SimilarityServiceImpl {
    fn get_similar_images(&self, image_id: &ImageId) -> Result<Vec<SimilarImage>, GetSimilarImagesError> {
        Ok(self.database.get_similar_images(image_id)?)
    }

    fn calculate_similarity_groups(&self) -> Result<Vec<Vec<ImageId>>, CalculateSimilarityGroupsError> {
        let image_pairs = self.database.get_all_similar_images()?;
        let image_groups = create_similar_groups_by_images(&image_pairs);
        let merged_groups = color_and_merge_groups(image_groups);
        Ok(merged_groups)
    }
}

fn create_similar_groups_by_images(image_pairs: &[SimilarImagePair]) -> HashMap<&ImageId, Vec<&ImageId>> {
    let mut image_groups = HashMap::new();
    for image_pair in image_pairs {
        {
            let group1 = image_groups.entry(&image_pair.image_id1).or_insert_with(HashSet::new);
            group1.insert(&image_pair.image_id1);
            group1.insert(&image_pair.image_id2);
        }
        {
            let group2 = image_groups.entry(&image_pair.image_id2).or_insert_with(HashSet::new);
            group2.insert(&image_pair.image_id1);
            group2.insert(&image_pair.image_id2);
        }
    }
    image_groups.into_iter()
        .map(|(image_id, group)| (image_id, group.into_iter().collect()))
        .collect()
}



#[cfg(test)]
mod test {
    use crate::similarity::SimilarityServiceImpl;
    use anyhow::Error;
    use kani_domain_api_incoming::similarity_service::SimilarityService;
    use kani_domain_api_model::image_id::ImageId;
    use kani_domain_api_model::similarity::{SimilarImage, SimilarImagePair};
    use kani_domain_api_outgoing::database::similarity_database::SimilarityDatabase;
    use std::sync::Arc;

    #[test]
    fn test_merge_chain_groups() {
        let service = SimilarityServiceImpl {
            database: Arc::new(MockDatabase(vec![(1,2), (2,3), (3,4), (4,5), (5,6)])),
        };

        let groups = service.calculate_similarity_groups().unwrap();

        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].len(), 6);
    }

    #[test]
    fn test_merge_split_groups() {
        let service = SimilarityServiceImpl {
            database: Arc::new(MockDatabase(vec![(1,2), (1,3), (2,3), (4,5), (5,6)])),
        };

        let groups = service.calculate_similarity_groups().unwrap();

        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].len(), 3);
        assert_eq!(groups[1].len(), 3);
    }

    #[test]
    fn test_merge_circular_groups() {
        let service = SimilarityServiceImpl {
            database: Arc::new(MockDatabase(vec![(1,2), (2,3), (3,4), (4,1)])),
        };

        let groups = service.calculate_similarity_groups().unwrap();

        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].len(), 4);
    }

    struct MockDatabase(Vec<(u8, u8)>);

    impl SimilarityDatabase for MockDatabase {
        fn get_similar_images(&self, _image_id: &ImageId) -> Result<Vec<SimilarImage>, Error> {
            todo!()
        }

        fn get_all_similar_images(&self) -> Result<Vec<SimilarImagePair>, Error> {
            Ok(
                self.0
                    .iter()
                    .map(|(id1, id2)| SimilarImagePair {
                        image_id1: ImageId([0,0,0,0,0,0,0,*id1]),
                        image_id2: ImageId([0,0,0,0,0,0,0,*id2]),
                        distance: 0,
                    })
                    .collect()
            )
        }
    }
}
