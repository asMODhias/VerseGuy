use crate::{aggregate::Organization, repo::OrganizationRepository};

pub struct OrganizationService<R> {
    repo: R,
}

impl<R> OrganizationService<R>
where
    R: OrganizationRepository + Send + Sync,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl<R> OrganizationService<R>
where
    R: OrganizationRepository + Send + Sync,
{
    pub async fn create_org(&self, org: &mut Organization) -> anyhow::Result<()> {
        self.repo.create(org).await?;
        Ok(())
    }

    pub async fn get(&self, id: &str) -> anyhow::Result<Option<Organization>> {
        self.repo.get_by_id(id).await
    }

    pub async fn delete_org(&self, id: &str) -> anyhow::Result<()> {
        self.repo.delete(id).await?;
        Ok(())
    }

    pub async fn add_member(&self, id: &str, member: crate::entity::Member) -> anyhow::Result<()> {
        let mut org = self
            .repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("not found"))?;
        org.add_member(member);
        self.repo.create(&mut org).await?;
        Ok(())
    }

    pub async fn deposit(&self, id: &str, amount: i64) -> anyhow::Result<()> {
        let mut org = self
            .repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("not found"))?;
        org.treasury.deposit(amount)?;
        self.repo.create(&mut org).await?;
        Ok(())
    }

    pub async fn withdraw(&self, id: &str, amount: i64) -> anyhow::Result<()> {
        let mut org = self
            .repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("not found"))?;
        org.treasury.withdraw(amount)?;
        self.repo.create(&mut org).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aggregate::Organization;
    use crate::entity::Member;
    use crate::value_object::OrganizationTag;
    use async_trait::async_trait;

    struct InMemoryRepo {
        store: std::sync::Mutex<Option<Organization>>,
    }

    impl InMemoryRepo {
        fn new() -> Self {
            Self {
                store: std::sync::Mutex::new(None),
            }
        }
    }

    #[async_trait]
    impl crate::repo::OrganizationRepository for InMemoryRepo {
        async fn create(&self, org: &mut Organization) -> anyhow::Result<()> {
            let mut s = self.store.lock().unwrap();
            *s = Some(org.clone());
            Ok(())
        }

        async fn get_by_id(&self, _id: &str) -> anyhow::Result<Option<Organization>> {
            let s = self.store.lock().unwrap();
            Ok(s.clone())
        }

        async fn delete(&self, _id: &str) -> anyhow::Result<()> {
            let mut s = self.store.lock().unwrap();
            *s = None;
            Ok(())
        }
    }

    #[tokio::test]
    async fn service_create_and_ops() -> anyhow::Result<()> {
        let repo = InMemoryRepo::new();
        let svc = OrganizationService::new(repo);

        let tag = OrganizationTag::new("ORG".to_string()).unwrap();
        let mut org = Organization::new(
            "id".to_string(),
            "TestOrg".to_string(),
            tag,
            "founder".to_string(),
        );

        svc.create_org(&mut org).await?;

        let loaded = svc.get(&org.id).await?.expect("exists");
        assert_eq!(loaded.name, "TestOrg");

        svc.deposit(&org.id, 100).await?;
        let loaded2 = svc.get(&org.id).await?.expect("exists");
        assert_eq!(loaded2.treasury.balance, 100);

        let member = Member::new("user-1".to_string(), &loaded2.ranks[0]);
        svc.add_member(&org.id, member).await?;
        let loaded3 = svc.get(&org.id).await?.expect("exists");
        assert_eq!(loaded3.members.len(), 1);

        svc.delete_org(&org.id).await?;
        assert!(svc.get(&org.id).await?.is_none());

        Ok(())
    }
}
