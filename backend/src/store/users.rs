impl crate::store::Store {
    #[tracing::instrument(name = "get_user_by_id", fields(user_id = id.to_string()))]
    pub async fn get_user_by_id(&self, id: uuid::Uuid) -> Result<crate::models::User, sqlx::Error> {
        sqlx::query_as::<_, crate::models::User>(
            r#"
        SELECT 
            *
        FROM users
        WHERE id = $1 AND is_active = true
        "#,
        )
        .bind(id)
        .fetch_one(&self.connection)
        .await
    }

    #[tracing::instrument(name = "get_user_by_email", fields(user_email = email))]
    pub async fn get_user_by_email(&self, email: &str) -> Result<crate::models::User, sqlx::Error> {
        sqlx::query_as::<_, crate::models::User>(
            r#"
        SELECT 
            *
        FROM users
        WHERE email = $1 AND is_active = true
        "#,
        )
        .bind(email)
        .fetch_one(&self.connection)
        .await
    }

    #[tracing::instrument(name = "create_user")]
    pub async fn create_user(
        &self,
        user_data: &crate::models::UserRegistration,
    ) -> Result<crate::models::User, sqlx::Error> {
        sqlx::query_as::<_, crate::models::User>(
            r#"
        INSERT INTO users (email, name, provider, is_active, is_staff, is_superuser, thumbnail)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING
            *
        "#,
        )
        .bind(&user_data.email)
        .bind(&user_data.name)
        .bind(&user_data.provider)
        .bind(user_data.is_active)
        .bind(user_data.is_staff)
        .bind(user_data.is_superuser)
        .bind(&user_data.thumbnail)
        .fetch_one(&self.connection)
        .await
    }

    #[tracing::instrument(name = "update_user")]
    pub async fn update_user(
        &self,
        user_id: &uuid::Uuid,
        user_data: &crate::models::UserRegistration,
    ) -> Result<crate::models::User, sqlx::Error> {
        sqlx::query_as::<_, crate::models::User>(
            r#"
        UPDATE users
        SET
            name = $1,
            email = $2,
            thumbnail = $3,
            provider = $4,
            is_active = $5,
            is_staff = $6,
            is_superuser = $7
        WHERE id = $8
        RETURNING
            *
        "#,
        )
        .bind(&user_data.name)
        .bind(&user_data.email)
        .bind(&user_data.thumbnail)
        .bind(&user_data.provider)
        .bind(user_data.is_active)
        .bind(user_data.is_staff)
        .bind(user_data.is_superuser)
        .bind(&user_id)
        .fetch_one(&self.connection)
        .await
    }

    #[tracing::instrument(name = "deactivate_user", fields(user_id = id.to_string()))]
    pub async fn deactivate_user(&self, id: &uuid::Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
        UPDATE users
        SET is_active = false
        WHERE id = $1
        "#,
        )
        .bind(id)
        .execute(&self.connection)
        .await?;

        Ok(())
    }
}
