#[cfg(test)]
mod tests {
  
    use crate::models::organization::CreateOrganizationRequest;
    use crate::db::organization::create_organization_db;
    use crate::db::tests::common::setup_test_db;

    #[tokio::test]
    async fn test_create_organization_db() {
        let pool = setup_test_db().await;
        
        let test_org = CreateOrganizationRequest {
            name: "Test Organization".to_string(),
        };

        let result = create_organization_db(&pool, test_org).await;
        assert!(result.is_ok());

        let created_org = result.unwrap();
        assert_eq!(created_org.name, "Test Organization");
    }

    #[tokio::test]
    async fn test_fetch_organization_db() {
        use crate::db::organization::fetch_organization_db;
        let pool = setup_test_db().await;

        let test_org = CreateOrganizationRequest {
            name: "Test Organization".to_string(),
        };

        let result = create_organization_db(&pool, test_org).await;
        assert!(result.is_ok());

        let created_org = result.unwrap();
        let fetched_org = fetch_organization_db(&pool, created_org.id).await;
        assert!(fetched_org.is_ok());

        let fetched_org = fetched_org.unwrap();
        assert_eq!(fetched_org.name, "Test Organization");

    }

    #[tokio::test]
    async fn test_update_organization_db() {
        use crate::models::organization::UpdateOrganizationRequest;
        use crate::db::organization::{update_organization_db,fetch_organization_db};
        let pool = setup_test_db().await;

        let test_org = CreateOrganizationRequest {
            name: "Test Organization".to_string(),
        };

        let result = create_organization_db(&pool, test_org).await;
        assert!(result.is_ok());

        let created_org = result.unwrap();
        let updated_org = UpdateOrganizationRequest {
            name: "Updated Organization".to_string(),
        };

        let result = update_organization_db(&pool, created_org.id, updated_org.name).await;
        assert!(result.is_ok());

        let rows_affected = result.unwrap();
        assert_eq!(rows_affected, 1);

        let fetched_org = fetch_organization_db(&pool, created_org.id).await;
        assert!(fetched_org.is_ok());

        let fetched_org = fetched_org.unwrap();
        assert_eq!(fetched_org.name, "Updated Organization");


    }

    #[tokio::test]
    async fn test_delete_organization_db() {
        use crate::db::organization::{delete_organization_db,fetch_organization_db};
        let pool = setup_test_db().await;

        let test_org = CreateOrganizationRequest {
            name: "Test Organization".to_string(),
        };

        let result = create_organization_db(&pool, test_org).await;
        assert!(result.is_ok());

        let created_org = result.unwrap();
        let result = delete_organization_db(&pool, created_org.id).await;
        assert!(result.is_ok());

        let rows_affected = result.unwrap();
        assert_eq!(rows_affected, 1);

        let fetched_org = fetch_organization_db(&pool, created_org.id).await;
        assert!(fetched_org.is_err());
    }

    #[tokio::test]
    async fn test_delete_organization_db_already_deleted() {
        use crate::db::organization::delete_organization_db;
        let pool = setup_test_db().await;

        let test_org = CreateOrganizationRequest {
            name: "Test Organization".to_string(),
        };

        let result = create_organization_db(&pool, test_org).await;
        assert!(result.is_ok());

        let created_org = result.unwrap();
        let result = delete_organization_db(&pool, created_org.id).await;
        assert!(result.is_ok());

        let rows_affected = result.unwrap();
        assert_eq!(rows_affected, 1);

        let result = delete_organization_db(&pool, created_org.id).await;
        assert!(result.is_ok());

        let rows_affected = result.unwrap();
        assert_eq!(rows_affected, 0);
    }

    #[tokio::test]
    async fn test_fetch_organization_db_deleted() {
        use crate::db::organization::delete_organization_db;
        use crate::db::organization::fetch_organization_db;
        let pool = setup_test_db().await;

        let test_org = CreateOrganizationRequest {
            name: "Test Organization".to_string(),
        };

        let result = create_organization_db(&pool, test_org).await;
        assert!(result.is_ok());

        let created_org = result.unwrap();
        let result = delete_organization_db(&pool, created_org.id).await;
        assert!(result.is_ok());

        let rows_affected = result.unwrap();
        assert_eq!(rows_affected, 1);

        let fetched_org = fetch_organization_db(&pool, created_org.id).await;
        assert!(fetched_org.is_err());
    }

    #[tokio::test]
    async fn test_fetch_organization_db_not_found() {
        use crate::db::organization::fetch_organization_db;
        let pool = setup_test_db().await;

        let fetched_org = fetch_organization_db(&pool, uuid::Uuid::new_v4()).await;
        assert!(fetched_org.is_err());
    }


}
