use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RevokedTokens::Table)
                    .if_not_exists()
                    // Le jti est l'identifiant unique du token révoqué
                    .col(ColumnDef::new(RevokedTokens::Jti).string().not_null().primary_key())
                    // L'id de l'utilisateur concerné (utile pour révoquer tous ses tokens)
                    .col(ColumnDef::new(RevokedTokens::UserId).uuid().not_null())
                    // Timestamp Unix d'expiration du token — permet la purge automatique
                    .col(ColumnDef::new(RevokedTokens::Exp).big_integer().not_null())
                    // Date d'ajout en blacklist
                    .col(ColumnDef::new(RevokedTokens::RevokedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RevokedTokens::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RevokedTokens {
    Table,
    Jti,
    UserId,
    Exp,
    RevokedAt,
}
